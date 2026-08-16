//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1183/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1183(t33913: f64, t7313: f64, t196: f64, t197: f64, t28230: f64, t2035: f64, t32103: f64, t4248: f64, t27123: f64, t8457: f64, t27126: f64, t32311: f64, t7732: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127314 = t33913 * t7313;
    let t127317 = t28230 * t196 * t197;
    let t127318 = t127317 * t2035;
    let t127324 = t4248 * t32103;
    let t127326 = t27123 * t8457;
    let t127328 = t27126 * t8457;
    let t127330 = t7732 * t32311;
    (t127314, t127318, t127324, t127326, t127328, t127330)
}
