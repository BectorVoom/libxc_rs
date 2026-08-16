//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2143/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2143(t11365: f64, t300: f64, t1714: f64, t44583: f64, t3447: f64, t3451: f64, t44584: f64, t4904: f64, t11588: f64, t4928: f64, t461: f64, t4729: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51848 = t300 * t11365;
    let t51968 = t44583 * t1714;
    let t51970 = t3447 * t51968 * t3451;
    let t51971 = 0.18518518518518518518e-3_f64 * t51970;
    let t51980 = t3447 * t44584 * t4904;
    let t51981 = 0.18518518518518518518e-3_f64 * t51980;
    let t52036 = t11588 * t4928;
    let t52057 = t3447 * t44583 * t461 * t4729;
    (t51848, t51968, t51971, t51981, t52036, t52057)
}
