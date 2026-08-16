//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2417/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2417(t2578: f64, t41189: f64, t9546: f64, t9555: f64, t2573: f64, t41008: f64, t2566: f64, t2570: f64, t9551: f64, t2588: f64, t40341: f64, t207: f64, t215: f64, t39933: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41190 = t41189 * t2578;
    let t41192 = t9546 * t9555;
    let t41194 = t41008 * t2573;
    let t41196 = t2566 * t2570;
    let t41197 = t41196 * t9551;
    let t41200 = 0.99537037037037037035e-1_f64 * t40341 * t2588;
    let t41209 = 0.14979423868312757201e0_f64 * t39933 * t207 * t215;
    (t41190, t41192, t41194, t41196, t41197, t41200, t41209)
}
