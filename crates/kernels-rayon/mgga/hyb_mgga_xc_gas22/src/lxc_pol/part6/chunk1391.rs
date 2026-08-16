//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1391/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1391(t30137: f64, t30150: f64, t30164: f64, t30177: f64, t949: f64, t968: f64, t21552: f64, t4273: f64, t21393: f64, t21396: f64, t21541: f64, t25214: f64, t25217: f64, t25220: f64, t29757: f64, t29760: f64, t29788: f64, t387: f64) -> (f64, f64, f64) {
    let t30182 = 1.0_f64 * t949 * (t30137 + t30150 + t30164 + t30177) * t968;
    let t30184 = 0.16081979498692535067e2_f64 * t21552 * t4273;
    let t30194 = (t21541 - 0.57685185185185185184e-1_f64 * t21393 + 0.12361111111111111111e-1_f64 * t21396 - 0.57685185185185185187e-1_f64 * t25214 + 0.49444444444444444446e-1_f64 * t25217 - 0.18541666666666666667e-1_f64 * t25220 + 0.12361111111111111111e-1_f64 * t29757 - 0.18541666666666666667e-1_f64 * t29760 + 0.278125e-1_f64 * t29788) * t387;
    (t30182, t30184, t30194)
}
