//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2070/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2070(t10970: f64, t820: f64, t10277: f64, t976: f64, t11046: f64, t42387: f64, t10457: f64, t10401: f64, t10935: f64, t3186: f64, t3200: f64, t10402: f64, t11034: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42397 = t820 * t10970;
    let t42444 = t976 * t10277;
    let t42483 = t11046 * t42387;
    let t42488 = t820 * t10457;
    let t42504 = t10935 * t10401;
    let t42505 = t3186 * t42504;
    let t42508 = t3200 * t42504;
    let t42541 = t11034 * t10402;
    (t42397, t42444, t42483, t42488, t42505, t42508, t42541)
}
