//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 913/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk913(t27634: f64, t7160: f64, t1078: f64, t11239: f64, t1035: f64, t1983: f64, t1668: f64, t1976: f64, t3153: f64, t4998: f64, t1043: f64, t1089: f64, t7828: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27635 = t7160 * t27634;
    let t27638 = t11239 * t1078;
    let t27639 = t27638 * t1035;
    let t27640 = t1983 * t27639;
    let t27641 = t1976 * t1668;
    let t27642 = t27641 * t3153;
    let t27643 = t27642 * t4998;
    let t27647 = t7828 * t1043 * t1089;
    (t27635, t27638, t27640, t27641, t27642, t27643, t27647)
}
