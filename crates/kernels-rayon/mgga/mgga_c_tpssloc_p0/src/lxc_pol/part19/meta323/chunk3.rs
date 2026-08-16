//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1146/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1146(t2471: f64, t118: f64, t181: f64, t2369: f64, t2460: f64, t2462: f64, t2477: f64, t2479: f64, t2494: f64, t2510: f64, t2512: f64, t39263: f64, t39283: f64, t39529: f64, t39549: f64, t39563: f64, t39585: f64, t39590: f64, t39593: f64, t39658: f64, t39664: f64, t730: f64, t731: f64, t745: f64, t747: f64, t9711: f64, t9730: f64, t9751: f64, t9752: f64, t9758: f64, t9762: f64, t9843: f64) -> f64 {
    let t39814 = t2471 * t2471;
    let t39840 = -8.0_f64 * t2460 * t9752 * t730 - 0.18989649058080861537e-2_f64 * t118 * t39283 * t181 + 0.69263436422725855036e2_f64 * t2510 * t9711 * t2512 * t745 + 0.96491876992155210402e2_f64 * t2477 * t39814 * t2479 + t39529 - 0.62337092780453269531e3_f64 * t9762 * t9843 * t2369 - 0.46785788981077169656e1_f64 * t2494 * t747 * t9711 + 36.0_f64 * t2477 * t2462 * t2471 - t39549 - t39563 + t39585 + 0.12865583598954028054e3_f64 * t2477 * t9751 * t2479 * t730 - t39590 + t39593 + 0.11579025239058625248e4_f64 * t9730 * t39664 * t2479 - 6.0_f64 * t2460 * t39814 * t731 + 0.61524113149298439947e4_f64 * t9758 * t39263 * t2369 + t39658;
    t39840
}
