//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2258/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2258(t3368: f64, t5277: f64, t1042: f64, t3704: f64, t5274: f64, t1774: f64, t3588: f64) -> (f64, f64, f64, f64) {
    let t17588 = t5277 * t3368;
    let t17589 = t1042 * t17588;
    let t17593 = 0.28582678745379824648e-3_f64 * t5274 * t3704;
    let t17600 = t1774 * t3588;
    (t17588, t17589, t17593, t17600)
}
