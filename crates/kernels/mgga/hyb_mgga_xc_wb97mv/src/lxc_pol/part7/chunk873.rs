//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 873/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk873<F: Float>(t7607: F, t7608: F, t1084: F, t2683: F, t2693: F, t1086: F, t2745: F, t471: F, t466: F, t2749: F, t7599: F, t1069: F, t1071: F, t1079: F, t221: F, t2730: F, t2741: F, t2747: F, t2764: F, t2771: F, t475: F, t488: F, t7479: F, t7509: F, t7546: F, t7559: F, t7562: F, t7565: F, t7566: F, t7580: F, t7589: F, t7592: F, t7593: F, t7598: F, t7602: F) -> (F, F, F, F, F, F, F) {
    let t7610 = 0.96491876992155210402e2 * t7607 * t7608;
    let t7612 = t2683 * t2693 * t1084;
    let t7615 = t1086 * t2683;
    let t7619 = 1.0 / t2745 / t471;
    let t7620 = t466 * t7619;
    let t7621 = t7599 * t2749;
    let t7624 = 0.96491876992155210402e2 * t2747 * t7546 * t1069 + t7479 - 6.0 * t2730 * t1071 * t2741 + 0.56968947174242584612e-3 * t221 * t7509 * t488 + 0.16562821945185185185e-2 * t221 * t7509 * t475 + 0.35089341735807877242e1 * t2771 * t7559 + 0.5848223622634646207e0 * t1079 * t7562 + 0.10254018858216406658e4 * t7565 * t7566 - t7580 - t7589 - 0.10389515463408878255e3 * t7592 * t7593 + 0.2069040516770936012e4 * t7598 * t7602 + t7610 + 0.51947577317044391277e2 * t2771 * t7612 - 0.35089341735807877242e1 * t2764 * t7615 - 0.19298375398431042081e3 * t7620 * t7621;
    (t7610, t7612, t7615, t7619, t7620, t7621, t7624)
}
