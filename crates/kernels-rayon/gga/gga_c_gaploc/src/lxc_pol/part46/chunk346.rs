//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 346/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk346(t1445: f64, t2846: f64, t1000: f64, t1004: f64, t1008: f64, t1013: f64, t1456: f64, t1580: f64, t1599: f64, t1641: f64, t193: f64, t2362: f64, t2369: f64, t2390: f64, t2411: f64, t2804: f64, t2807: f64, t2810: f64, t2816: f64, t2819: f64, t2823: f64, t2828: f64, t2834: f64, t2843: f64, t541: f64, t557: f64, t574: f64, t597: f64) -> f64 {
    let t2847 = t1445 * t2846;
    let t2850 = 0.30674340763136599741e1_f64 * t597 * t2804 - 0.23833659967900284446e0_f64 * t557 * t2807 - 0.30674340763136599741e1_f64 * t574 * t2810 + 0.23833659967900284446e0_f64 * t1000 * t541 + 0.23005755572352449806e1_f64 * t597 * t2816 + 0.35750489951850426669e0_f64 * t2819 * t193 + 0.35750489951850426669e0_f64 * t2823 * t193 - 0.35750489951850426669e0_f64 * t1599 * t1004 - 0.35750489951850426669e0_f64 * t557 * t2828 - 0.23005755572352449806e1_f64 * t1641 * t1008 - 0.23005755572352449806e1_f64 * t574 * t2834 + 0.23005755572352449806e1_f64 * t1580 * t1013 + 0.25561950635947166451e0_f64 * t2362 - 0.29792074959875355558e-1_f64 * t2369 - 0.59584149919750711116e-1_f64 * t2390 + 0.29792074959875355558e-1_f64 * t2411 + 0.35750489951850426669e0_f64 * t1456 * t2843 - 0.46011511144704899612e1_f64 * t574 * t2847;
    t2850
}
