//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2418/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2418(t207: f64, t40344: f64, t795: f64, t116: f64, t786: f64, t9534: f64, t133: f64, t6600: f64, t776: f64, t39568: f64, t761: f64, t2535: f64, t9716: f64) -> (f64, f64, f64, f64, f64) {
    let t41212 = 0.11265432098765432099e0_f64 * t40344 * t207 * t795;
    let t41214 = t9534 * t786 * t116;
    let t41217 = t41214 * t133 * t6600 * t776;
    let t41254 = 0.14035736694323150897e2_f64 * t761 * t39568;
    let t41255 = t9716 * t2535;
    (t41212, t41214, t41217, t41254, t41255)
}
