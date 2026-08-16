//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2006/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2006(t7284: f64, t94377: f64, t2453: f64, t555: f64, t25898: f64, t136: f64, t137: f64, t2022: f64, t1399: f64, t2438: f64, t25304: f64, t25876: f64, t25931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t94378 = t7284 * t94377;
    let t94382 = t2453 * t555;
    let t94383 = t94382 * t25898;
    let t94385 = t2022 * t136 * t137;
    let t94386 = t2438 * t1399;
    let t94387 = t94385 * t94386;
    let t94388 = t94383 * t94387;
    let t94390 = t25304 * t555;
    let t94391 = t94390 * t25898;
    let t94392 = t94391 * t94387;
    let t94394 = t25876 * t25931;
    (t94378, t94382, t94383, t94385, t94388, t94390, t94391, t94392, t94394)
}
