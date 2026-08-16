//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 927/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk927(t230: f64, t4977: f64, t2440: f64, t4939: f64, t39976: f64, t5249: f64, t703: f64, t1196: f64, t2725: f64, t800: f64, t2035: f64, t5009: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t70290 = t230 * t4977;
    let t70326 = t2440 * t4939;
    let t70354 = 0.59031789687271907073e-3_f64 * t39976 * t5249;
    let t70402 = t703 * t4977;
    let t70462 = t2725 * t1196;
    let t70463 = t800 * t70462;
    let t70474 = t2035 * t5009;
    (t70290, t70326, t70354, t70402, t70462, t70463, t70474)
}
