//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1146/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1146<F: Float>(t2471: F, t118: F, t181: F, t2369: F, t2460: F, t2462: F, t2477: F, t2479: F, t2494: F, t2510: F, t2512: F, t39263: F, t39283: F, t39529: F, t39549: F, t39563: F, t39585: F, t39590: F, t39593: F, t39658: F, t39664: F, t730: F, t731: F, t745: F, t747: F, t9711: F, t9730: F, t9751: F, t9752: F, t9758: F, t9762: F, t9843: F) -> F {
    let t39814 = t2471 * t2471;
    let t39840 = -F::cast_from(8.0_f64) * t2460 * t9752 * t730 - F::cast_from(0.18989649058080861537e-2_f64) * t118 * t39283 * t181 + F::cast_from(0.69263436422725855036e2_f64) * t2510 * t9711 * t2512 * t745 + F::cast_from(0.96491876992155210402e2_f64) * t2477 * t39814 * t2479 + t39529 - F::cast_from(0.62337092780453269531e3_f64) * t9762 * t9843 * t2369 - F::cast_from(0.46785788981077169656e1_f64) * t2494 * t747 * t9711 + F::cast_from(36.0_f64) * t2477 * t2462 * t2471 - t39549 - t39563 + t39585 + F::cast_from(0.12865583598954028054e3_f64) * t2477 * t9751 * t2479 * t730 - t39590 + t39593 + F::cast_from(0.11579025239058625248e4_f64) * t9730 * t39664 * t2479 - F::cast_from(6.0_f64) * t2460 * t39814 * t731 + F::cast_from(0.61524113149298439947e4_f64) * t9758 * t39263 * t2369 + t39658;
    t39840
}
