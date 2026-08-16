//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1366/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1366(t265: f64, t502: f64, t105669: f64, t112958: f64, t114089: f64, t116331: f64, t116381: f64, t116430: f64, t116469: f64, t116520: f64, t116565: f64, t116607: f64, t116649: f64, t1300: f64, t1832: f64, t198: f64, t24501: f64, t25026: f64, t27041: f64, t29317: f64, t336: f64, t5023: f64, t6748: f64, t6752: f64, t7673: f64, t97498: f64) -> f64 {
    let t503 = t265 < t502;
    let t116675 = piecewise3(t503, t198 * t336 * (t116331 + t116381 + t116430 + t116469 + t116520 + t116565 + t116607 + t116649) * t1300 - 3.0_f64 * t5023 * t112958 * t1832 + 6.0_f64 * t5023 * t105669 * t6752 - 3.0_f64 * t5023 * t29317 * t6748 - 6.0_f64 * t5023 * t97498 * t24501 + 6.0_f64 * t5023 * t27041 * t1832 * t6748 - t5023 * t7673 * t25026, t114089);
    t116675
}
