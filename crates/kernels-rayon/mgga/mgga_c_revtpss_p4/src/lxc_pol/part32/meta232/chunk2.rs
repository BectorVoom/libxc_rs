//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 992/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk992(t6258: f64, t996: f64, t1592: f64, t4823: f64, t1042: f64, t1469: f64, t3094: f64, t4781: f64, t3092: f64, t1651: f64, t1668: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6259 = t996 * t6258;
    let t6262 = t4823 * t1592;
    let t6263 = t1042 * t6262;
    let t6266 = t3094 * t1469;
    let t6267 = t4781 * t6266;
    let t6268 = t3092 * t6267;
    let t6271 = t1651 * t1668;
    (t6259, t6262, t6263, t6266, t6267, t6268, t6271)
}
