//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1368/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1368(t2783: f64, t9801: f64, t10745: f64, t10698: f64, t10895: f64, t2394: f64, t2430: f64, t2477: f64, t2730: f64, t40232: f64, t40240: f64, t40462: f64, t40471: f64, t40473: f64, t40475: f64, t40477: f64, t40482: f64, t40484: f64, t40489: f64, t40491: f64, t40503: f64, t40507: f64, t40509: f64, t40511: f64, t775: f64, t800: f64, t825: f64, t827: f64, t828: f64, t851: f64) -> f64 {
    let t40517 = t9801 * t2783;
    let t40518 = t40517 * t10745;
    let t40520 = 0.18007087609589289528e0_f64 * t851 * t40462 * t828 * t40232 + 0.12862205435420921092e-1_f64 * t851 * t2477 * t828 * t40240 + 0.48018900292238105409e0_f64 * t40471 - 0.45732285992607719437e-3_f64 * t40473 - 0.45732285992607719437e-3_f64 * t40475 + 0.32524801797942610064e-2_f64 * t40477 + 0.28582678745379824648e-4_f64 * t40482 + 0.16006300097412701803e-1_f64 * t40484 + 0.28900264064772933811e-2_f64 * t40489 - 0.64311027177104605458e-3_f64 * t825 * t827 * t828 * t40491 - 0.1543464652250510531e0_f64 * t851 * t10698 * t828 * t2394 * t2430 + 0.34299214494455789577e-2_f64 * t40503 + t40507 + 0.15246000842785598467e-4_f64 * t40509 - 7.0_f64 / 4.0_f64 * t40511 + t2730 * t800 * t10895 * t775 / 4.0_f64 - 0.18295201011342718161e-3_f64 * t40518;
    t40520
}
