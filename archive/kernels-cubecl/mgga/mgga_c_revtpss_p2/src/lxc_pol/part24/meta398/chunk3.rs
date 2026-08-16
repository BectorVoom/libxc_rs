//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1328/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1328<F: Float>(t123: F, t164: F, t177: F, t186: F, t215: F, t2492: F, t2514: F, t2535: F, t2549: F, t2552: F, t2553: F, t2554: F, t2556: F, t2557: F, t2591: F, t268: F, t39500: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t39875: F, t39886: F, t39960: F, t39963: F, t39967: F, t39989: F, t730: F, t745: F, t9371: F, t9433: F, t9454: F, t9480: F, t9485: F, t9530: F, t9532: F, t9536: F) -> F {
    let t40007 = t39791 + t39795 - t39799 - t39807 + t39813 + F::cast_from(0.91082604192152556044e5_f64) * t177 * t39960 * t39875 * t39963 + F::cast_from(0.11579025239058625248e4_f64) * t9530 * t39967 * t2556 - F::cast_from(0.14035736694323150897e2_f64) * t9480 * t39875 * t745 + F::cast_from(0.96491876992155210402e2_f64) * t2554 * t39886 * t2556 - F::cast_from(24.0_f64) * t9433 * t39967 * t730 - F::cast_from(0.24828486201251232145e5_f64) * t164 / t2552 / t2535 * t39967 * t9532 + t39989 + F::cast_from(0.61524113149298439947e4_f64) * t9536 * t2492 * t9371 * t2514 - F::cast_from(0.18989649058080861537e-2_f64) * t123 * t39500 * t186 - F::cast_from(0.21687162600603479684e-1_f64) * t268 * t2591 * t9485 + F::cast_from(0.13698666666666666666e0_f64) * t268 * t9454 * t2549 + F::cast_from(0.44060335298551228073e1_f64) * t268 * t215 * t2553 * t2557;
    t40007
}
