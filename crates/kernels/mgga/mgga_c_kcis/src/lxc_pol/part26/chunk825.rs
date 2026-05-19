//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 825/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk825<F: Float>(t2011: F, t4134: F, t3728: F, t5882: F, t5678: F, t1494: F, t2001: F, t13396: F, t1392: F, t86: F, t5782: F, t2007: F, t3245: F) -> (F, F, F, F, F, F, F, F) {
    let t15910 = t4134 * t2011;
    let t15934 = t3728 * t5882;
    let t15941 = t3728 * t5678;
    let t15942 = F::cast_from(0.66327777777777777776e-2_f64) * t15941;
    let t15955 = t1494 * t2001;
    let t15967 = t86 * t13396 * t1392;
    let t15968 = t15967 * t5782;
    let t15983 = t3245 * t2007;
    (t15910, t15934, t15941, t15942, t15955, t15967, t15968, t15983)
}
