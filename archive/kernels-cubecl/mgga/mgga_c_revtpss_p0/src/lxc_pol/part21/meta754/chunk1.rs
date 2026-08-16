//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2639/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2639<F: Float>(t2661: F, t3938: F, t3992: F, t48533: F, t14045: F, t9810: F, t13774: F, t1399: F, t13927: F, t48100: F, t9816: F, t1353: F, t13716: F, t13789: F, t1410: F, t3934: F, t4012: F, t46660: F, t48466: F, t48494: F, t48498: F, t48509: F, t48510: F, t48514: F, t48516: F, t48518: F, t48527: F, t48529: F, t48532: F, t5671: F, t5673: F, t5674: F, t828: F, t9912: F) -> F {
    let t48536 = t2661 * t3992 * t48533 * t3938;
    let t48540 = t2661 * t3992 * t14045 * t9810;
    let t48544 = t2661 * t3992 * t13774 * t1399;
    let t48548 = t9816 * t48100 * t13927;
    let t48550 = F::cast_from(0.15246000842785598468e-3_f64) * t48494 - F::cast_from(0.76230004213927992338e-3_f64) * t48498 + F::cast_from(0.30011812682648815881e-2_f64) * t5671 * t5673 * t5674 * t9912 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t13789 * t48466 * t1399 - t48509 - F::cast_from(0.12004725073059526352e0_f64) * t48510 - F::cast_from(0.42874018118069736972e-4_f64) * t48514 + F::cast_from(0.91464571985215438874e-3_f64) * t48516 + F::cast_from(0.37792653007779990369e-1_f64) * t48518 + F::cast_from(0.12862205435420921092e-1_f64) * t1410 * t4012 * t828 * t13716 * t1353 + F::cast_from(0.15246000842785598468e-2_f64) * t48527 + F::cast_from(0.91464571985215438873e-3_f64) * t48529 - t48532 - F::cast_from(0.17149607247227894789e-3_f64) * t48536 - F::cast_from(0.85748036236139473944e-4_f64) * t48540 + F::cast_from(0.42874018118069736972e-4_f64) * t48544 + F::cast_from(0.24009450146119052705e-1_f64) * t46660 + F::cast_from(0.30492001685571196935e-3_f64) * t48548;
    t48550
}
