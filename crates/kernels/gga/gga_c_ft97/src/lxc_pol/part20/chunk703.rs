//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 703/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk703<F: Float>(t2749: F, t4167: F, t840: F, t4129: F, t875: F, t871: F, t1212: F, t2801: F, t1882: F, t4164: F, t4169: F, t2862: F, t4162: F, t882: F, t824: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15154 = t840 * t2749 * t4167;
    let t15157 = t4129 * t875;
    let t15159 = t840 * t871 * t15157;
    let t15162 = t1212 * t2801;
    let t15164 = t840 * t871 * t15162;
    let t15168 = 4.0 / 9.0 * t1882 * t4164;
    let t15170 = 2.0 / 9.0 * t1882 * t4169;
    let t15172 = t2862 * t882 * t4162;
    let t15175 = t4129 * t824;
    (t15154, t15157, t15159, t15162, t15164, t15168, t15170, t15172, t15175)
}
