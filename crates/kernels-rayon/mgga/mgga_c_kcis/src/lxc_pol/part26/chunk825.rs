//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 825/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk825(t2011: f64, t4134: f64, t3728: f64, t5882: f64, t5678: f64, t1494: f64, t2001: f64, t13396: f64, t1392: f64, t86: f64, t5782: f64, t2007: f64, t3245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15910 = t4134 * t2011;
    let t15934 = t3728 * t5882;
    let t15941 = t3728 * t5678;
    let t15942 = 0.66327777777777777776e-2_f64 * t15941;
    let t15955 = t1494 * t2001;
    let t15967 = t86 * t13396 * t1392;
    let t15968 = t15967 * t5782;
    let t15983 = t3245 * t2007;
    (t15910, t15934, t15941, t15942, t15955, t15967, t15968, t15983)
}
