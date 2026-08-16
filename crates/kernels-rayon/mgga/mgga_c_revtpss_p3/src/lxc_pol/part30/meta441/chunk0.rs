//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1693/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1693(t17330: f64, t459: f64, t225: f64, t480: f64, t1256: f64, t5258: f64, t5262: f64, t1804: f64, t3655: f64, t1786: f64, t1260: f64, t12987: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17331 = t17330 * t459;
    let t17332 = t17331 * t225;
    let t17333 = t17332 * t480;
    let t17337 = 0.15244095330869239812e-2_f64 * t5258 * t1256;
    let t17339 = 0.28582678745379824648e-3_f64 * t5262 * t1256;
    let t17340 = t1804 * t3655;
    let t17342 = t1786 * t3655;
    let t17344 = t12987 * t1260;
    (t17331, t17332, t17333, t17337, t17339, t17340, t17342, t17344)
}
