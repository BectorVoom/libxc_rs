//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1112/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1112(t676: f64, t837: f64, t25377: f64, t25411: f64, t2718: f64, t867: f64, t1949: f64, t2722: f64, t2723: f64, t1950: f64, t2453: f64, t2458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25412 = t676 * t837;
    let t25413 = t25377 * t25412;
    let t25414 = t25411 * t25413;
    let t25416 = t867 * t2718;
    let t25417 = t1949 * t2722;
    let t25418 = t25417 * t2723;
    let t25419 = t25416 * t25418;
    let t25422 = t2453 * t1950;
    let t25424 = 0.11565819519348392139e-2_f64 * t25422 * t2458;
    (t25412, t25413, t25414, t25416, t25417, t25418, t25419, t25422, t25424)
}
