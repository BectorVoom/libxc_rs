//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2639/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2639(t2661: f64, t3938: f64, t3992: f64, t48533: f64, t14045: f64, t9810: f64, t13774: f64, t1399: f64, t13927: f64, t48100: f64, t9816: f64, t1353: f64, t13716: f64, t13789: f64, t1410: f64, t3934: f64, t4012: f64, t46660: f64, t48466: f64, t48494: f64, t48498: f64, t48509: f64, t48510: f64, t48514: f64, t48516: f64, t48518: f64, t48527: f64, t48529: f64, t48532: f64, t5671: f64, t5673: f64, t5674: f64, t828: f64, t9912: f64) -> f64 {
    let t48536 = t2661 * t3992 * t48533 * t3938;
    let t48540 = t2661 * t3992 * t14045 * t9810;
    let t48544 = t2661 * t3992 * t13774 * t1399;
    let t48548 = t9816 * t48100 * t13927;
    let t48550 = 0.15246000842785598468e-3_f64 * t48494 - 0.76230004213927992338e-3_f64 * t48498 + 0.30011812682648815881e-2_f64 * t5671 * t5673 * t5674 * t9912 + 0.25724410870841842183e-2_f64 * t3934 * t13789 * t48466 * t1399 - t48509 - 0.12004725073059526352e0_f64 * t48510 - 0.42874018118069736972e-4_f64 * t48514 + 0.91464571985215438874e-3_f64 * t48516 + 0.37792653007779990369e-1_f64 * t48518 + 0.12862205435420921092e-1_f64 * t1410 * t4012 * t828 * t13716 * t1353 + 0.15246000842785598468e-2_f64 * t48527 + 0.91464571985215438873e-3_f64 * t48529 - t48532 - 0.17149607247227894789e-3_f64 * t48536 - 0.85748036236139473944e-4_f64 * t48540 + 0.42874018118069736972e-4_f64 * t48544 + 0.24009450146119052705e-1_f64 * t46660 + 0.30492001685571196935e-3_f64 * t48548;
    t48550
}
