//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1254/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1254(t1593: f64, t28374: f64, t3999: f64, t7908: f64, t28343: f64, t94246: f64, t27369: f64, t1014: f64, t28525: f64, t27484: f64, t8151: f64, t28473: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98845 = t7908 * t1593 * t3999 * t28374;
    let t98847 = t94246 * t28343;
    let t98849 = 0.46336805555555555556e-3_f64 * t7908 * t98847;
    let t98854 = 0.61836467013888888889e-4_f64 * t27369 * t98847;
    let t98863 = t1014 * t28525;
    let t98864 = 0.33163888888888888888e-2_f64 * t98863;
    let t98874 = t8151 * t27484;
    let t98887 = t1014 * t28473;
    (t98845, t98849, t98854, t98863, t98864, t98874, t98887)
}
