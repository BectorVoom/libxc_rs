//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1189/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1189(t1008: f64, t6361: f64, t1163: f64, t1166: f64, t20417: f64, t1096: f64, t1165: f64, t12401: f64, t1426: f64, t1531: f64, t16314: f64, t16707: f64, t175: f64, t20400: f64, t21607: f64, t21609: f64, t21611: f64, t21613: f64, t21615: f64, t21620: f64, t3084: f64, t335: f64, t336: f64, t418: f64, t4450: f64, t4463: f64, t495: f64, t5852: f64) -> f64 {
    let t21625 = t1008 * t6361;
    let t21632 = t1163 * t20417 * t1166;
    let t21642 = -0.42874018118069736972e-3_f64 * t16707 - 0.17149607247227894789e-1_f64 * t4463 * t1165 * t20400 * t1096 - 0.85748036236139473944e-3_f64 * t21607 - 0.42874018118069736972e-3_f64 * t21609 - 0.34299214494455789578e-2_f64 * t21611 + 0.85748036236139473944e-2_f64 * t21613 + 0.85748036236139473944e-2_f64 * t418 * t1426 * t175 * t21615 + 0.42874018118069736972e-2_f64 * t418 * t1426 * t175 * t21620 - 0.51448821741683684366e-1_f64 * t21625 - t335 * t336 * t16314 * t495 / 24.0_f64 + 0.85748036236139473944e-3_f64 * t21632 - 0.12862205435420921092e-2_f64 * t4450 * t1165 * t5852 * t12401 + 0.12862205435420921092e-2_f64 * t1531 * t1165 * t5852 * t3084;
    t21642
}
