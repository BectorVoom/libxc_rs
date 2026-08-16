//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1373/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1373<F: Float>(t33436: F, t33441: F, t33454: F, t33457: F, t33460: F, t33462: F, t33466: F, t33468: F, t33470: F, t33472: F, t33477: F, t33479: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36587 = F::cast_from(0.18937162934584967535e-3_f64) * t33436;
    let t36588 = F::cast_from(0.18937162934584967535e-3_f64) * t33441;
    let t36593 = F::cast_from(0.91900712057578208105e-2_f64) * t33454;
    let t36594 = F::cast_from(0.11594181388521408695e-4_f64) * t33457;
    let t36596 = F::cast_from(0.33764099580923002116e-6_f64) * t33460;
    let t36597 = F::cast_from(0.45018799441230669488e-6_f64) * t33462;
    let t36599 = F::cast_from(0.70341874126922921074e-8_f64) * t33466;
    let t36600 = F::cast_from(0.15458908518028544927e-5_f64) * t33468;
    let t36601 = F::cast_from(0.2748593934505475288e-5_f64) * t33470;
    let t36602 = F::cast_from(0.22509399720615334744e-6_f64) * t33472;
    let t36604 = F::cast_from(0.9275345110817126956e-4_f64) * t33477;
    let t36605 = F::cast_from(0.9275345110817126956e-4_f64) * t33479;
    (t36587, t36588, t36593, t36594, t36596, t36597, t36599, t36600, t36601, t36602, t36604, t36605)
}
