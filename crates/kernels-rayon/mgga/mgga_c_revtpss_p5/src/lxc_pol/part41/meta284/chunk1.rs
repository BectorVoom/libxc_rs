//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1041/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1041(t10744: f64, t10745: f64, t2693: f64, t2710: f64, t2713: f64, t810: f64, t9784: f64, t9789: f64, t235: f64, t2783: f64, t2453: f64, t2664: f64, t9794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10746 = t10744 * t10745;
    let t10749 = t2710 * t2713 * t2693;
    let t10756 = 0.72250660161932334527e-3_f64 * t9784 * t810;
    let t10758 = 0.11294745624363664198e-6_f64 * t9789 * t810;
    let t10759 = t2783 * t235;
    let t10760 = t2453 * t10759;
    let t10761 = t9794 * t2664;
    (t10746, t10749, t10756, t10758, t10760, t10761)
}
