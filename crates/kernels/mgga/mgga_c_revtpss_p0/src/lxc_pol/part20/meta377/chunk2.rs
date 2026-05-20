//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1368/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1368<F: Float>(t2783: F, t9801: F, t10745: F, t10698: F, t10895: F, t2394: F, t2430: F, t2477: F, t2730: F, t40232: F, t40240: F, t40462: F, t40471: F, t40473: F, t40475: F, t40477: F, t40482: F, t40484: F, t40489: F, t40491: F, t40503: F, t40507: F, t40509: F, t40511: F, t775: F, t800: F, t825: F, t827: F, t828: F, t851: F) -> F {
    let t40517 = t9801 * t2783;
    let t40518 = t40517 * t10745;
    let t40520 = F::cast_from(0.18007087609589289528e0_f64) * t851 * t40462 * t828 * t40232 + F::cast_from(0.12862205435420921092e-1_f64) * t851 * t2477 * t828 * t40240 + F::cast_from(0.48018900292238105409e0_f64) * t40471 - F::cast_from(0.45732285992607719437e-3_f64) * t40473 - F::cast_from(0.45732285992607719437e-3_f64) * t40475 + F::cast_from(0.32524801797942610064e-2_f64) * t40477 + F::cast_from(0.28582678745379824648e-4_f64) * t40482 + F::cast_from(0.16006300097412701803e-1_f64) * t40484 + F::cast_from(0.28900264064772933811e-2_f64) * t40489 - F::cast_from(0.64311027177104605458e-3_f64) * t825 * t827 * t828 * t40491 - F::cast_from(0.1543464652250510531e0_f64) * t851 * t10698 * t828 * t2394 * t2430 + F::cast_from(0.34299214494455789577e-2_f64) * t40503 + t40507 + F::cast_from(0.15246000842785598467e-4_f64) * t40509 - F::new(7.0) / F::new(4.0) * t40511 + t2730 * t800 * t10895 * t775 / F::new(4.0) - F::cast_from(0.18295201011342718161e-3_f64) * t40518;
    t40520
}
