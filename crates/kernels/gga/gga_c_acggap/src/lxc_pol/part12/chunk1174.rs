//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1174/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1174<F: Float>(t34534: F, t34537: F, t34547: F, t34549: F, t34556: F, t30624: F, t34522: F, t34526: F, t34529: F, t34532: F, t34539: F, t34541: F, t34543: F, t34545: F, t34553: F, t34559: F, t34561: F, t34563: F) -> F {
    let t37140 = F::new(0.34299214494455789578e-2) * t34534;
    let t37142 = F::new(0.17149607247227894789e-2) * t34537;
    let t37147 = F::new(0.34299214494455789578e-2) * t34547;
    let t37148 = F::new(0.16006300097412701803e-1) * t34549;
    let t37150 = F::new(0.12579236915841660828e-2) * t34556;
    let t37154 = F::new(0.37737710747524982482e-2) * t34522 + F::new(0.83861579438944405518e-3) * t34526 + t34529 / F::new(24.0) + t34532 / F::new(24.0) - t37140 + F::new(0.85748036236139473944e-3) * t30624 + t37142 - F::new(0.34299214494455789578e-2) * t34539 + F::new(0.51448821741683684367e-2) * t34541 - F::new(0.34299214494455789578e-1) * t34543 + F::new(0.10289764348336736873e-1) * t34545 - t37147 - t37148 + F::new(0.18868855373762491241e-2) * t34553 + t37150 + F::new(0.62896184579208304138e-2) * t34559 + F::new(0.37737710747524982482e-2) * t34561 + F::new(0.27439371595564631662e-1) * t34563;
    t37154
}
