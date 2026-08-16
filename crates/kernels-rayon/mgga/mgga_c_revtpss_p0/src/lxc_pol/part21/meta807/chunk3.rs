//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2946/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2946(t3162: f64, t999: f64, t11722: f64, t4834: f64, t11727: f64, t16143: f64, t3127: f64, t3172: f64, t15772: f64, t3106: f64, t15775: f64, t1042: f64, t11160: f64, t15611: f64, t15725: f64, t15728: f64, t15839: f64, t15893: f64, t16149: f64, t3117: f64, t42346: f64, t42643: f64, t43044: f64, t4823: f64) -> f64 {
    let t53619 = t3162 * t999;
    let t53626 = t4834 * t11722;
    let t53628 = t4834 * t11727;
    let t53633 = t3127 * t3172 * t16143;
    let t53641 = t3106 * t15772;
    let t53643 = t3106 * t15775;
    let t53645 = 0.28582678745379824648e-3_f64 * t42346 - 0.12862205435420921092e-2_f64 * t43044 * t3117 * t15893 * t53619 - 0.25724410870841842183e-2_f64 * t42643 * t15611 + 0.28582678745379824648e-3_f64 * t53626 + 0.47637797908966374413e-3_f64 * t53628 + 0.12862205435420921092e-2_f64 * t15725 * t15839 + 0.57165357490759649295e-3_f64 * t53633 - 0.85748036236139473944e-3_f64 * t3127 * t1042 * t4823 * t11160 - 0.45732285992607719436e-2_f64 * t15728 * t16149 + 0.60976381323476959248e-2_f64 * t53641 - 0.5081365110289746604e-2_f64 * t53643;
    t53645
}
