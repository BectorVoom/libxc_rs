//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 914/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk914(t27426: f64, t7160: f64, t1043: f64, t1089: f64, t7817: f64, t7821: f64, t1096: f64, t7810: f64, t988: f64, t7145: f64, t4820: f64, t7122: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27427 = t7160 * t27426;
    let t27433 = t7817 * t1043 * t1089;
    let t27437 = t7821 * t1043 * t1089;
    let t27440 = t7810 * t1096;
    let t27441 = t7160 * t27440;
    let t27444 = t7810 * t988;
    let t27445 = t7145 * t27444;
    let t27448 = t7122 * t4820;
    (t27427, t27433, t27437, t27441, t27445, t27448)
}
