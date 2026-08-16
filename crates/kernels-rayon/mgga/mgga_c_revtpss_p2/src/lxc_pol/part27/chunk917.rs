//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 917/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk917(t2408: f64, t2411: f64, t262: f64, t775: f64, t10566: f64, t10568: f64, t10570: f64, t10575: f64, t10577: f64, t10580: f64, t10582: f64, t10584: f64, t2403: f64, t2430: f64, t4541: f64, t9514: f64, t9517: f64, t9521: f64) -> f64 {
    let t11084 = t2408 * t2411;
    let t11088 = t262 * t775;
    let t11092 = -9.0_f64 * t11084 * t2403 * t775 + 18.0_f64 * t11088 * t2430 * t4541 + t10566 - t10568 + t10570 - t10575 + t10577 + t10580 + t10582 - t10584 + t9514 - t9517 - t9521;
    t11092
}
