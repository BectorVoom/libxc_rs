//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1297/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1297(t10001: f64, t10003: f64, t1419: f64, t4086: f64, t786: f64, t4104: f64, t268: f64, t4056: f64, t543: f64, t675: f64, t4101: f64, t555: f64, t5744: f64) -> (f64, f64, f64, f64, f64) {
    let t10004 = t10001 * t10003;
    let t10013 = t4086 * t1419;
    let t10014 = t786 * t10013;
    let t10015 = t10014 * t4104;
    let t10019 = t268 * t675 * t4056 * t543;
    let t10020 = t4101 * t10019;
    let t10022 = t5744 * t555;
    (t10004, t10014, t10015, t10020, t10022)
}
