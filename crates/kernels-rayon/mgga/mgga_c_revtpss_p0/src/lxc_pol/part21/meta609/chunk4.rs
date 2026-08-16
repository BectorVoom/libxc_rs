//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2355/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2355(t123: f64, t164: f64, t177: f64, t186: f64, t215: f64, t2492: f64, t2514: f64, t2535: f64, t2549: f64, t2552: f64, t2553: f64, t2554: f64, t2556: f64, t2557: f64, t2591: f64, t268: f64, t39500: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64, t39875: f64, t39886: f64, t39960: f64, t39963: f64, t39967: f64, t39989: f64, t730: f64, t745: f64, t9371: f64, t9433: f64, t9454: f64, t9480: f64, t9485: f64, t9530: f64, t9532: f64, t9536: f64) -> f64 {
    let t40007 = t39791 + t39795 - t39799 - t39807 + t39813 + 0.91082604192152556044e5_f64 * t177 * t39960 * t39875 * t39963 + 0.11579025239058625248e4_f64 * t9530 * t39967 * t2556 - 0.14035736694323150897e2_f64 * t9480 * t39875 * t745 + 0.96491876992155210402e2_f64 * t2554 * t39886 * t2556 - 24.0_f64 * t9433 * t39967 * t730 - 0.24828486201251232145e5_f64 * t164 / t2552 / t2535 * t39967 * t9532 + t39989 + 0.61524113149298439947e4_f64 * t9536 * t2492 * t9371 * t2514 - 0.18989649058080861537e-2_f64 * t123 * t39500 * t186 - 0.21687162600603479684e-1_f64 * t268 * t2591 * t9485 + 0.13698666666666666666e0_f64 * t268 * t9454 * t2549 + 0.44060335298551228073e1_f64 * t268 * t215 * t2553 * t2557;
    t40007
}
