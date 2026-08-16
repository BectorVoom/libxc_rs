//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1944/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1944(t18657: f64, t1955: f64, t1579: f64, t231: f64, t4423: f64, t25207: f64, t77441: f64, t1544: f64, t580: f64, t98646: f64, t18435: f64, t27159: f64) -> (f64, f64, f64, f64, f64) {
    let t106404 = t1955 * t18657;
    let t106410 = t1579 * t4423 * t231;
    let t106490 = t25207 * t77441;
    let t106494 = t98646 * t580 * t1544;
    let t106498 = t27159 * t18435;
    (t106404, t106410, t106490, t106494, t106498)
}
