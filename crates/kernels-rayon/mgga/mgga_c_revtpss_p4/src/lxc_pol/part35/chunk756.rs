//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 756/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk756(t810: f64, t9784: f64, t9789: f64, t235: f64, t2783: f64, t2453: f64, t2475: f64, t72: f64, t245: f64, t2482: f64, t814: f64, t823: f64) -> (f64, f64, f64, f64, f64) {
    let t10756 = 0.72250660161932334527e-3_f64 * t9784 * t810;
    let t10758 = 0.11294745624363664198e-6_f64 * t9789 * t810;
    let t10759 = t2783 * t235;
    let t10760 = t2453 * t10759;
    let t10769 = t2475 * t72;
    let t10770 = t10769 * t245;
    let t10777 = t2482 * t823 * t814;
    (t10756, t10758, t10760, t10770, t10777)
}
