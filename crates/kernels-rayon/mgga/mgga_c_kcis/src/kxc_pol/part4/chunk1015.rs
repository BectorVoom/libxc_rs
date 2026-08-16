//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1015/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1015(t4066: f64, t738: f64, t4069: f64, t4099: f64, t4102: f64, t743: f64, t531: f64, t822: f64, t3110: f64, t317: f64, t522: f64, t323: f64, t526: f64, t8291: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12011 = t738 * t4066;
    let t12013 = t738 * t4069;
    let t12015 = t738 * t4099;
    let t12026 = t743 * t4102;
    let t12048 = t822 * t531;
    let t12049 = 0.62154466893555682512e-3_f64 * t12048;
    let t12058 = 0.27323333333333333333e-1_f64 * t317 * t3110 * t522;
    let t12061 = 0.77488888888888888888e-2_f64 * t323 * t8291 * t526;
    (t12011, t12013, t12015, t12026, t12048, t12049, t12058, t12061)
}
