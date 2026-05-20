//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2000/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2000<F: Float>(t102980: F, t93190: F, t10073: F, t26554: F, t27198: F, t102972: F, t25411: F, t15003: F, t95773: F, t15030: F, t25391: F, t26550: F, t28425: F, t7403: F, t95624: F, t95629: F, t95632: F, t95635: F, t95645: F, t95647: F, t95649: F, t95651: F, t99309: F, t99369: F) -> F {
    let t103009 = t93190 * t102980;
    let t103017 = t10073 * t27198 * t26554;
    let t103023 = F::cast_from(0.25702851531048074406e-1_f64) * t25411 * t102972;
    let t103030 = t95773 * t15003;
    let t103033 = F::cast_from(0.45699670022203476294e-2_f64) * t103009 + F::cast_from(0.51405703062096148812e-1_f64) * t95624 - F::cast_from(0.68540937416128198416e-1_f64) * t95629 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t26550 * t99309 + t95632 - F::cast_from(0.24093411633903331839e-3_f64) * t103017 + F::cast_from(0.17347256376410398924e1_f64) * t25391 * t28425 * t99369 + t103023 - F::cast_from(0.54878743191129263322e-2_f64) * t95635 + F::cast_from(0.26341796731742046394e1_f64) * t7403 * t15030 + F::cast_from(0.72280234901709995518e-2_f64) * t95645 - F::cast_from(0.12851425765524037203e-1_f64) * t95647 + F::cast_from(0.25702851531048074406e-1_f64) * t95649 - F::cast_from(0.11565819519348392139e-2_f64) * t103030 - F::cast_from(0.9757440539382783019e-2_f64) * t95651;
    t103033
}
