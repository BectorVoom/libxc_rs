//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1211/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1211(t10588: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t10592: f64, t11084: f64, t14385: f64, t14388: f64, t14392: f64, t14396: f64, t14397: f64, t14428: f64, t14433: f64, t1544: f64, t1940: f64, t2394: f64, t2403: f64, t4541: f64, t4546: f64, t890: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64) -> (f64, f64) {
    let t14434 = 0.5848223622634646207e0_f64 * t10588;
    let t14435 = -3.0_f64 * t11084 * t1544 * t2403 - 2.0_f64 * t14397 * t1940 * t890 + 6.0_f64 * t2394 * t4541 * t4546 + t10577 + t10582 - t10584 - t10586 + t10592 + t14385 + t14388 + t14392 + t14396 + t14428 + t14433 - t14434 + t9514 - t9517 - t9521 - t9524;
    (t14434, t14435)
}
