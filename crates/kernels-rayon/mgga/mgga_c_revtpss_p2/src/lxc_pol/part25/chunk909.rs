//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 909/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk909(t810: f64, t9784: f64, t9789: f64, t235: f64, t2783: f64, t2453: f64, t2664: f64, t9794: f64, t125: f64, t2430: f64, t2747: f64, t837: f64) -> (f64, f64, f64, f64, f64) {
    let t10756 = 0.72250660161932334527e-3_f64 * t9784 * t810;
    let t10758 = 0.11294745624363664198e-6_f64 * t9789 * t810;
    let t10759 = t2783 * t235;
    let t10760 = t2453 * t10759;
    let t10761 = t9794 * t2664;
    let t10762 = t10760 * t10761;
    let t10764 = t125 * t2430;
    let t10766 = t2747 * t10764 * t837;
    (t10756, t10758, t10761, t10762, t10766)
}
