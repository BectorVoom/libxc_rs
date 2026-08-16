//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 962/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk962(t1131: f64, t14595: f64, t1096: f64, t1092: f64, t3182: f64, t4823: f64, t4819: f64, t9532: f64, t4793: f64, t9429: f64, t2861: f64, t4815: f64) -> (f64, f64, f64, f64, f64) {
    let t14596 = t1131 * t14595;
    let t14597 = t1096 * t14596;
    let t14598 = t1092 * t14597;
    let t14600 = t3182 * t4823;
    let t14601 = t1096 * t14600;
    let t14602 = t1092 * t14601;
    let t14604 = t9532 * t4819;
    let t14605 = t1092 * t14604;
    let t14607 = t9429 * t4793;
    let t14609 = t2861 * t4815;
    (t14598, t14602, t14605, t14607, t14609)
}
