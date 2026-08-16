//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1403/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1403(t10769: f64, t828: f64, t1544: f64, t836: f64, t2746: f64, t2710: f64, t2713: f64, t4371: f64, t4353: f64, t808: f64, t10744: f64, t10905: f64, t4442: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14785 = t10769 * t828;
    let t14786 = t1544 * t836;
    let t14791 = t2746 * t828;
    let t14817 = t2710 * t2713 * t4371;
    let t14819 = t808 * t4353;
    let t14820 = t10744 * t14819;
    let t14823 = 7.0_f64 / 24.0_f64 * t10905 * t4442;
    (t14785, t14786, t14791, t14817, t14820, t14823)
}
