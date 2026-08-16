//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1505/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1505(t6075: f64, t892: f64, t262: f64, t5962: f64, t10568: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t14353: f64, t14433: f64, t1544: f64, t18557: f64, t18558: f64, t18561: f64, t18564: f64, t18565: f64, t18567: f64, t2403: f64, t2404: f64, t4541: f64, t775: f64, t9514: f64, t9517: f64, t9521: f64) -> f64 {
    let t18850 = t6075 * t892;
    let t18860 = t262 * t5962;
    let t18864 = 6.0_f64 * t14353 * t1544 * t2403 + 3.0_f64 * t18850 * t2403 * t775 + 6.0_f64 * t18860 * t4541 * t775 + 3.0_f64 * t2403 * t2404 * t5962 - t10568 + t10577 + t10582 - t10584 - t10586 + t14433 - t18557 + t18558 + t18561 - t18564 + t18565 + t18567 + t9514 - t9517 - t9521;
    t18864
}
