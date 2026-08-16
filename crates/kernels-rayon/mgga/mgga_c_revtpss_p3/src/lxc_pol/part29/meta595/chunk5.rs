//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2000/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2000(t102980: f64, t93190: f64, t10073: f64, t26554: f64, t27198: f64, t102972: f64, t25411: f64, t15003: f64, t95773: f64, t15030: f64, t25391: f64, t26550: f64, t28425: f64, t7403: f64, t95624: f64, t95629: f64, t95632: f64, t95635: f64, t95645: f64, t95647: f64, t95649: f64, t95651: f64, t99309: f64, t99369: f64) -> f64 {
    let t103009 = t93190 * t102980;
    let t103017 = t10073 * t27198 * t26554;
    let t103023 = 0.25702851531048074406e-1_f64 * t25411 * t102972;
    let t103030 = t95773 * t15003;
    let t103033 = 0.45699670022203476294e-2_f64 * t103009 + 0.51405703062096148812e-1_f64 * t95624 - 0.68540937416128198416e-1_f64 * t95629 - 0.8673628188205199462e0_f64 * t25391 * t26550 * t99309 + t95632 - 0.24093411633903331839e-3_f64 * t103017 + 0.17347256376410398924e1_f64 * t25391 * t28425 * t99369 + t103023 - 0.54878743191129263322e-2_f64 * t95635 + 0.26341796731742046394e1_f64 * t7403 * t15030 + 0.72280234901709995518e-2_f64 * t95645 - 0.12851425765524037203e-1_f64 * t95647 + 0.25702851531048074406e-1_f64 * t95649 - 0.11565819519348392139e-2_f64 * t103030 - 0.9757440539382783019e-2_f64 * t95651;
    t103033
}
