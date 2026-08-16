//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2286/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2286(t5427: f64, t608: f64, t5392: f64, t9287: f64, t607: f64, t3966: f64, t3981: f64, t2267: f64, t5398: f64, t16558: f64, t43: f64, t9300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19363 = t608 * t5427;
    let t19368 = t9287 * t5392;
    let t19369 = t19368 * t607;
    let t19372 = t3981 * t3966;
    let t19377 = t2267 * t5398;
    let t19378 = t19377 * t607;
    let t19381 = t43 * t16558;
    let t19390 = t9300 * t5392;
    (t19363, t19368, t19369, t19372, t19377, t19378, t19381, t19390)
}
