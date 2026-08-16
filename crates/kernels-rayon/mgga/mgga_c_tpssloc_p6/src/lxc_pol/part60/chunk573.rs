//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 573/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk573(t50: f64, t6794: f64, t131: f64, t467: f64, t1009: f64, t461: f64, t1209: f64, t475: f64, t68: f64, t2157: f64, t3640: f64, t112: f64, t2169: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7371 = t50 * t6794;
    let t7372 = t7371 * t131;
    let t7373 = t7372 * t467;
    let t7374 = t461 * t1009;
    let t7375 = t7374 * t1209;
    let t7376 = t68 * t475;
    let t7398 = t2157 * t3640;
    let t7423 = t2169 * t112;
    (t7371, t7372, t7373, t7375, t7376, t7398, t7423)
}
