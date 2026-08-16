//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1219/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1219(t28189: f64, t8764: f64, t32855: f64, t7732: f64, t2322: f64, t34382: f64, t125491: f64, t125495: f64, t125497: f64, t125499: f64, t125500: f64, t125502: f64, t125505: f64, t125507: f64, t125510: f64, t34394: f64, t649: f64) -> f64 {
    let t129322 = t8764 * t28189;
    let t129326 = t7732 * t32855;
    let t129328 = t2322 * t34382;
    let t129330 = -t34394 * t649 - t125491 + t125495 + 6.0_f64 * t125497 - t125499 - t125500 + 3.0_f64 * t125502 - t125505 - t125507 + t125510 - t129322 - 2.0_f64 * t129326 - 2.0_f64 * t129328;
    t129330
}
