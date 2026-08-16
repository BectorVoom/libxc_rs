//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1329/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1329<F: Float>(t123: F, t173: F, t2514: F, t2536: F, t2538: F, t2548: F, t2549: F, t2553: F, t2556: F, t2597: F, t2604: F, t2605: F, t39490: F, t39492: F, t39495: F, t39498: F, t39500: F, t39501: F, t39506: F, t39508: F, t39510: F, t39512: F, t39515: F, t39747: F, t39750: F, t39756: F, t39760: F, t39773: F, t39783: F, t39786: F, t39815: F, t689: F, t724: F, t729: F, t730: F, t744: F, t9318: F, t9323: F, t9480: F, t9485: F, t9530: F, t9532: F) -> F {
    let t39957 = -F::cast_from(0.55209406483950617283e-2_f64) * t123 * t39500 * t173 - F::cast_from(0.46785788981077169656e1_f64) * t2597 * t9485 * t744 - t39747 - t39750 - t39756 - t39760 - F::cast_from(0.19263893255070628431e1_f64) * t689 * t9323 + F::cast_from(0.41096e0_f64) * t689 * t2536 * t729 * t2549 - F::cast_from(0.6609050294782684211e1_f64) * t689 * t2553 * t2548 * t2556 * t729 + F::cast_from(0.1301229756036208781e0_f64) * t689 * t9318 + F::cast_from(1.0_f64) * t724 * (-F::cast_from(0.39219166666666666667e1_f64) * t39490 + F::cast_from(0.376504e2_f64) * t39492 - F::cast_from(0.13944592592592592593e2_f64) * t39495 + F::cast_from(0.12201518518518518519e2_f64) * t39498 + F::cast_from(0.5356037037037037037e1_f64) * t39501 + F::cast_from(0.14025833333333333333e0_f64) * t39506 - F::cast_from(0.22441333333333333332e1_f64) * t39508 + F::cast_from(0.24934814814814814815e1_f64) * t39510 + F::cast_from(0.21817962962962962963e1_f64) * t39512 + F::cast_from(0.16979925925925925926e1_f64) * t39515) * t730 + F::cast_from(0.69263436422725855036e2_f64) * t2604 * t39815 * t744 - t39773 + F::cast_from(0.12414243100625616072e5_f64) * t9530 * t2548 * t9532 * t2538 - F::cast_from(0.62337092780453269531e3_f64) * t9480 * t2605 * t2514 + t39783 + t39786;
    t39957
}
