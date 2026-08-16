//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1115/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1115(t10867: f64, t867: f64, t25410: f64, t93189: f64, t25374: f64, t93169: f64, t2453: f64, t555: f64, t25898: f64, t25304: f64, t2482: f64, t7262: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93355 = t867 * t10867;
    let t93371 = t93189 * t25410;
    let t93377 = t93169 * t25374;
    let t94382 = t2453 * t555;
    let t94383 = t94382 * t25898;
    let t94390 = t25304 * t555;
    let t94391 = t94390 * t25898;
    let t94423 = t2482 * t7262 * t814;
    (t93355, t93371, t93377, t94382, t94383, t94390, t94391, t94423)
}
