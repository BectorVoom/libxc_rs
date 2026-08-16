//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 376/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk376(t1720: f64, t491: f64, t1196: f64, t1409: f64, t974: f64, t225: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t1721 = t1720 * t491;
    let t1725 = t1196 * t1409;
    let t1726 = t974 * t1725;
    let t1729 = t1720 * t225;
    let t1730 = t1729 * t68;
    (t1721, t1725, t1726, t1729, t1730)
}
