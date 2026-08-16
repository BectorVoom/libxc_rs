//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1016/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1016(t265: f64, t502: f64, t34943: f64, t34994: f64, t1300: f64, t1832: f64, t198: f64, t33533: f64, t33539: f64, t336: f64, t33866: f64, t5023: f64, t7673: f64, t8220: f64) -> (f64, f64) {
    let t503 = t265 < t502;
    let t34995 = t34943 + t34994;
    let t35008 = piecewise3(t503, t1300 * t198 * t336 * t34995 - t1832 * t33533 * t5023 + 2.0_f64 * t1832 * t33539 * t5023 - 2.0_f64 * t5023 * t7673 * t8220, t33866);
    (t34995, t35008)
}
