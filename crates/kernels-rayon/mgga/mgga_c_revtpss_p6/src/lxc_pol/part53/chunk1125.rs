//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1125/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1125(t125209: f64, t508: f64, t1310: f64, t33639: f64, t1843: f64, t32171: f64, t5517: f64, t8454: f64, t32322: f64, t7935: f64, t28076: f64, t8441: f64, t8621: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125211 = 2.0_f64 * t125209 * t508;
    let t125213 = 2.0_f64 * t33639 * t1310;
    let t125215 = 2.0_f64 * t32171 * t1843;
    let t125217 = 2.0_f64 * t8454 * t5517;
    let t125223 = t32322 * t7935;
    let t125238 = t8621 * t8441 * t28076;
    (t125211, t125213, t125215, t125217, t125223, t125238)
}
