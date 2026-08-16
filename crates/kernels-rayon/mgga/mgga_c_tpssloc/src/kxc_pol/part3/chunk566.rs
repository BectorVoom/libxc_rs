//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 566/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk566(t225: f64, t2666: f64, t68: f64, t845: f64, t2379: f64, t2553: f64, t824: f64, t228: f64, t230: f64, t822: f64, t825: f64, t232: f64) -> (f64, f64, f64, f64, f64) {
    let t2667 = t2666 * t225;
    let t2671 = t68 * t845;
    let t2672 = t2671 * t2379;
    let t2675 = t824 * t2553;
    let t2678 = -12.0_f64 * t228 * t2672 + 3.0_f64 * t228 * t2675 - t230 * t2667 + 6.0_f64 * t822 * t825;
    let t2679 = t2678 * t232;
    (t2667, t2672, t2675, t2678, t2679)
}
