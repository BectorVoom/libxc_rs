//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2784/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2784(t14803: f64, t14931: f64, t51123: f64, t4372: f64, t9784: f64, t2475: f64, t808: f64, t14787: f64, t50768: f64, t10627: f64, t10818: f64, t10872: f64, t14586: f64, t14691: f64, t14785: f64, t14791: f64, t14802: f64, t14894: f64, t1548: f64, t1559: f64, t18632: f64, t2645: f64, t2724: f64, t2745: f64, t2747: f64, t2749: f64, t2754: f64, t36833: f64, t40560: f64, t40862: f64, t40865: f64, t40868: f64, t4362: f64, t4364: f64, t4365: f64, t4450: f64, t50418: f64, t50474: f64, t51122: f64, t51125: f64, t51135: f64, t800: f64, t836: f64) -> f64 {
    let t51168 = t14931 * t51123 * t14803;
    let t51170 = t9784 * t4372;
    let t51176 = t808 * t2475;
    let t51178 = t50768 * t51176 * t14787;
    let t51180 = 0.25724410870841842183e-2_f64 * t2745 * t2747 * t14691 * t2754 + t51122 + 0.30492001685571196935e-3_f64 * t51125 - 0.38586616306262763276e-2_f64 * t14894 * t36833 * t50474 * t2645 * t836 + 0.34299214494455789577e-3_f64 * t51135 - 0.77173232612525526552e-2_f64 * t14894 * t4364 * t4365 * t10872 + 0.51448821741683684367e-2_f64 * t14894 * t2747 * t4450 * t10872 - 0.12862205435420921092e-1_f64 * t2745 * t14785 * t1559 * t10818 + 5.0_f64 / 4.0_f64 * t40868 * t800 * t1548 * t10627 + 455.0_f64 / 216.0_f64 * t40862 + 7.0_f64 / 12.0_f64 * t40865 - 0.12862205435420921092e-1_f64 * t2745 * t14785 * t50418 * t2749 - 0.10289764348336736873e-1_f64 * t4362 * t14791 * t18632 * t14802 - 0.51448821741683684367e-2_f64 * t4362 * t14791 * t14586 * t40560 - 0.6098400337114239387e-3_f64 * t51168 + 0.28900264064772933812e-2_f64 * t51170 - 0.51448821741683684367e-2_f64 * t4362 * t2747 * t14691 * t2724 + 0.85748036236139473944e-3_f64 * t51178;
    t51180
}
