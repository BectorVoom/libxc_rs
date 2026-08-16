//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 410/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk410(t2580: f64, t3447: f64, t2508: f64, t1052: f64, t977: f64, t3040: f64, t955: f64, t2976: f64, t959: f64, t1645: f64, t948: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3448 = t2580 * t3447;
    let t3450 = 0.15381052460284448567e-1_f64 * t2508 * t3448;
    let t3459 = t1052 * t977;
    let t3463 = 0.35750489951850426669e0_f64 * t955 * t3040;
    let t3468 = t2976 * t959;
    let t3469 = 0.14896037479937677779e-1_f64 * t3468;
    let t3470 = t1645 * t948;
    (t3448, t3450, t3459, t3463, t3469, t3470)
}
