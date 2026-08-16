//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1174/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1174(t34534: f64, t34537: f64, t34547: f64, t34549: f64, t34556: f64, t30624: f64, t34522: f64, t34526: f64, t34529: f64, t34532: f64, t34539: f64, t34541: f64, t34543: f64, t34545: f64, t34553: f64, t34559: f64, t34561: f64, t34563: f64) -> f64 {
    let t37140 = 0.34299214494455789578e-2_f64 * t34534;
    let t37142 = 0.17149607247227894789e-2_f64 * t34537;
    let t37147 = 0.34299214494455789578e-2_f64 * t34547;
    let t37148 = 0.16006300097412701803e-1_f64 * t34549;
    let t37150 = 0.12579236915841660828e-2_f64 * t34556;
    let t37154 = 0.37737710747524982482e-2_f64 * t34522 + 0.83861579438944405518e-3_f64 * t34526 + t34529 / 24.0_f64 + t34532 / 24.0_f64 - t37140 + 0.85748036236139473944e-3_f64 * t30624 + t37142 - 0.34299214494455789578e-2_f64 * t34539 + 0.51448821741683684367e-2_f64 * t34541 - 0.34299214494455789578e-1_f64 * t34543 + 0.10289764348336736873e-1_f64 * t34545 - t37147 - t37148 + 0.18868855373762491241e-2_f64 * t34553 + t37150 + 0.62896184579208304138e-2_f64 * t34559 + 0.37737710747524982482e-2_f64 * t34561 + 0.27439371595564631662e-1_f64 * t34563;
    t37154
}
