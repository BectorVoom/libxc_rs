//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 556/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk556(t210: f64, t2605: f64, t119: f64, t2553: f64, t225: f64, t2591: f64, t237: f64, t68: f64, t808: f64) -> (f64, f64, f64, f64, f64) {
    let t2606 = t210 * t2605;
    let t2610 = t210 * t119 * t2553;
    let t2613 = t2591 * t225;
    let t2614 = t2613 * t237;
    let t2617 = t808 * t68;
    (t2606, t2610, t2613, t2614, t2617)
}
