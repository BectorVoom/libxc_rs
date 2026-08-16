//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1734/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1734(t12032: f64, t225: f64, t385: f64, t3270: f64, t999: f64, t3269: f64, t11804: f64, t996: f64, t1035: f64, t11239: f64) -> (f64, f64, f64, f64, f64) {
    let t12034 = t12032 * t225 * t385;
    let t12039 = t999 * t3270;
    let t12040 = t3269 * t12039;
    let t12043 = t996 * t11804;
    let t12046 = t11239 * t1035;
    (t12034, t12039, t12040, t12043, t12046)
}
