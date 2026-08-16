//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1047/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1047(t14457: f64, t2515: f64, t141: f64, t3431: f64, t3748: f64, t4573: f64, t8493: f64, t581: f64, t8633: f64, t4826: f64, t861: f64, t3753: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14461 = t2515 * t14457;
    let t14462 = t141 * t14461;
    let t14464 = t3748 * t3431;
    let t14465 = t2515 * t14464;
    let t14466 = t141 * t14465;
    let t14468 = t8493 * t4573;
    let t14469 = t14468 * t581;
    let t14470 = t8633 * t14469;
    let t14471 = t141 * t14470;
    let t14473 = t4826 * t581;
    let t14474 = t861 * t14473;
    let t14475 = t141 * t14474;
    let t14477 = t3753 * t3431;
    (t14462, t14464, t14466, t14469, t14471, t14473, t14475, t14477)
}
