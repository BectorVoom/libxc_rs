//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 912/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk912(t342: f64, t7810: f64, t1678: f64, t3140: f64, t1078: f64, t1982: f64, t1089: f64, t1668: f64, t25681: f64, t4866: f64, t7168: f64, t7828: f64, t988: f64) -> (f64, f64, f64, f64, f64) {
    let t27616 = t342 * t7810;
    let t27619 = t1678 * t3140;
    let t27621 = t1982 * t27619 * t1078;
    let t27627 = t25681 * t1668 * t1089;
    let t27631 = t7168 * t4866 * t1089;
    let t27634 = t7828 * t988;
    (t27616, t27621, t27627, t27631, t27634)
}
