//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2784/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2784<F: Float>(t14803: F, t14931: F, t51123: F, t4372: F, t9784: F, t2475: F, t808: F, t14787: F, t50768: F, t10627: F, t10818: F, t10872: F, t14586: F, t14691: F, t14785: F, t14791: F, t14802: F, t14894: F, t1548: F, t1559: F, t18632: F, t2645: F, t2724: F, t2745: F, t2747: F, t2749: F, t2754: F, t36833: F, t40560: F, t40862: F, t40865: F, t40868: F, t4362: F, t4364: F, t4365: F, t4450: F, t50418: F, t50474: F, t51122: F, t51125: F, t51135: F, t800: F, t836: F) -> F {
    let t51168 = t14931 * t51123 * t14803;
    let t51170 = t9784 * t4372;
    let t51176 = t808 * t2475;
    let t51178 = t50768 * t51176 * t14787;
    let t51180 = F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t2747 * t14691 * t2754 + t51122 + F::cast_from(0.30492001685571196935e-3_f64) * t51125 - F::cast_from(0.38586616306262763276e-2_f64) * t14894 * t36833 * t50474 * t2645 * t836 + F::cast_from(0.34299214494455789577e-3_f64) * t51135 - F::cast_from(0.77173232612525526552e-2_f64) * t14894 * t4364 * t4365 * t10872 + F::cast_from(0.51448821741683684367e-2_f64) * t14894 * t2747 * t4450 * t10872 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t14785 * t1559 * t10818 + F::cast_from(5.0_f64) / F::cast_from(4.0_f64) * t40868 * t800 * t1548 * t10627 + F::cast_from(455.0_f64) / F::cast_from(216.0_f64) * t40862 + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t40865 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t14785 * t50418 * t2749 - F::cast_from(0.10289764348336736873e-1_f64) * t4362 * t14791 * t18632 * t14802 - F::cast_from(0.51448821741683684367e-2_f64) * t4362 * t14791 * t14586 * t40560 - F::cast_from(0.6098400337114239387e-3_f64) * t51168 + F::cast_from(0.28900264064772933812e-2_f64) * t51170 - F::cast_from(0.51448821741683684367e-2_f64) * t4362 * t2747 * t14691 * t2724 + F::cast_from(0.85748036236139473944e-3_f64) * t51178;
    t51180
}
