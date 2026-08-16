//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1344/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1344(t16708: f64, t16710: f64, t16712: f64, t1256: f64, t5258: f64, t5262: f64, t1804: f64, t3655: f64, t1786: f64, t1260: f64, t12987: f64, t15687: f64, t3623: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17319 = 0.37037037037037037037e-2_f64 * t16708;
    let t17320 = 0.11111111111111111111e-1_f64 * t16710;
    let t17321 = 0.55555555555555555556e-2_f64 * t16712;
    let t17337 = 0.15244095330869239812e-2_f64 * t5258 * t1256;
    let t17339 = 0.28582678745379824648e-3_f64 * t5262 * t1256;
    let t17340 = t1804 * t3655;
    let t17342 = t1786 * t3655;
    let t17344 = t12987 * t1260;
    let t17350 = t3623 * t15687;
    (t17319, t17320, t17321, t17337, t17339, t17340, t17342, t17344, t17350)
}
