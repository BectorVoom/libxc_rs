//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 642/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk642(t225: f64, t3727: f64, t494: f64, t1269: f64, t460: f64, t1275: f64, t493: f64, t1294: f64, t1204: f64, t1284: f64, t1280: f64, t3568: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3729 = t3727 * t225 * t494;
    let t3732 = t460 * t1269;
    let t3736 = 1.0_f64 / t1275 / t493;
    let t3737 = t225 * t3736;
    let t3738 = t1294 * t1294;
    let t3739 = t3737 * t3738;
    let t3746 = t1204 * t1284;
    let t3751 = t1280 * t3568;
    (t3729, t3732, t3737, t3738, t3739, t3746, t3751)
}
