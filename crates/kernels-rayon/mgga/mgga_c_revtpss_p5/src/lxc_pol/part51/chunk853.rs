//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 853/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk853(t1096: f64, t7817: f64, t7160: f64, t7821: f64, t988: f64, t7145: f64, t1035: f64, t7810: f64, t1043: f64, t1089: f64, t1982: f64, t27418: f64) -> (f64, f64, f64, f64) {
    let t27594 = t7817 * t1096;
    let t27595 = t7160 * t27594;
    let t27598 = t7821 * t988;
    let t27599 = t7145 * t27598;
    let t27604 = t1035 * t7810;
    let t27606 = t27604 * t1043 * t1089;
    let t27609 = t1982 * t27418;
    (t27595, t27599, t27606, t27609)
}
