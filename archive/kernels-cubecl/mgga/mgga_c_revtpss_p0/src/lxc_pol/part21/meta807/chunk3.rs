//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2946/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2946<F: Float>(t3162: F, t999: F, t11722: F, t4834: F, t11727: F, t16143: F, t3127: F, t3172: F, t15772: F, t3106: F, t15775: F, t1042: F, t11160: F, t15611: F, t15725: F, t15728: F, t15839: F, t15893: F, t16149: F, t3117: F, t42346: F, t42643: F, t43044: F, t4823: F) -> F {
    let t53619 = t3162 * t999;
    let t53626 = t4834 * t11722;
    let t53628 = t4834 * t11727;
    let t53633 = t3127 * t3172 * t16143;
    let t53641 = t3106 * t15772;
    let t53643 = t3106 * t15775;
    let t53645 = F::cast_from(0.28582678745379824648e-3_f64) * t42346 - F::cast_from(0.12862205435420921092e-2_f64) * t43044 * t3117 * t15893 * t53619 - F::cast_from(0.25724410870841842183e-2_f64) * t42643 * t15611 + F::cast_from(0.28582678745379824648e-3_f64) * t53626 + F::cast_from(0.47637797908966374413e-3_f64) * t53628 + F::cast_from(0.12862205435420921092e-2_f64) * t15725 * t15839 + F::cast_from(0.57165357490759649295e-3_f64) * t53633 - F::cast_from(0.85748036236139473944e-3_f64) * t3127 * t1042 * t4823 * t11160 - F::cast_from(0.45732285992607719436e-2_f64) * t15728 * t16149 + F::cast_from(0.60976381323476959248e-2_f64) * t53641 - F::cast_from(0.5081365110289746604e-2_f64) * t53643;
    t53645
}
