//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1517/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1517(t2710: f64, t2713: f64, t4371: f64, t4353: f64, t808: f64, t10744: f64, t10905: f64, t4442: f64, t4457: f64, t775: f64, t800: f64, t1548: f64, t2430: f64) -> (f64, f64, f64, f64, f64) {
    let t14817 = t2710 * t2713 * t4371;
    let t14819 = t808 * t4353;
    let t14820 = t10744 * t14819;
    let t14823 = 7.0_f64 / 24.0_f64 * t10905 * t4442;
    let t14825 = t800 * t4457 * t775;
    let t14829 = t800 * t1548 * t2430;
    (t14817, t14820, t14823, t14825, t14829)
}
