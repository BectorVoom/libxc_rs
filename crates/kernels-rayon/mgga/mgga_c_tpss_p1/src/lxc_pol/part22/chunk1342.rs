//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1342/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1342(t20509: f64, t2436: f64, t6353: f64, t8096: f64, t1692: f64, t17938: f64, t18053: f64, t18056: f64, t18059: f64, t1812: f64, t18728: f64, t18807: f64, t19678: f64, t19821: f64, t20514: f64, t20526: f64, t2439: f64, t5591: f64, t5853: f64, t62610: f64, t6354: f64, t63771: f64, t63791: f64, t63806: f64, t63837: f64, t63845: f64, t64256: f64, t64267: f64, t64297: f64) -> (f64, f64, f64) {
    let t66281 = t20509 * t2436;
    let t66299 = t6353 * t8096;
    let t66302 = 2.0_f64 * t20526 * t64267 - 3.0_f64 / 2.0_f64 * t18728 * t64297 + 3.0_f64 / 2.0_f64 * t2439 * t6354 * t17938 - t1692 * t18807 * t19821 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t63806 - t1692 * t20514 * t18056 - t1692 * t66281 * t5591 - t1692 * t5853 * t63791 - 3.0_f64 * t62610 * t19678 + t20526 * t63845 + 2.0_f64 * t20526 * t63837 - t1692 * t20514 * t18059 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t18728 * t64256 - t1692 * t5853 * t63771 / 2.0_f64 + t1692 * t66299 * t18053;
    (t66281, t66299, t66302)
}
