//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1084/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1084(t3316: f64, t3353: f64, t4114: f64, t6497: f64, t2183: f64, t4140: f64, t4117: f64, t6585: f64, t791: f64, t3324: f64, t3329: f64, t2194: f64, t4121: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10561 = 2.0_f64 * t3316 * t3353;
    let t10563 = 2.0_f64 * t6497 * t4114;
    let t10565 = 1.0_f64 * t2183 * t4140;
    let t10566 = t6585 * t4117;
    let t10567 = t10566 * t791;
    let t10569 = t3324 * t3329;
    let t10571 = t2194 * t4121;
    (t10561, t10563, t10565, t10566, t10567, t10569, t10571)
}
