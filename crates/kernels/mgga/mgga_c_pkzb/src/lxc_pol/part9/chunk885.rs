//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 885/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk885<F: Float>(t2312: F, t2317: F, t3161: F, t898: F, t2328: F, t2332: F, t6122: F, t890: F, t6116: F, t6196: F, t6204: F, t6207: F, t6319: F, t6322: F, t6329: F, t6333: F, t6358: F) -> (F, F, F, F, F, F) {
    let t6496 = t2317 * t2312 * t3161;
    let t6498 = F::new(0.51947577317044391277e2) * t898 * t6496;
    let t6500 = F::new(0.35089341735807877242e1) * t2328 * t2332;
    let t6502 = t2317 * t6122 * t890;
    let t6504 = F::new(0.35089341735807877242e1) * t898 * t6502;
    let t6505 = t6196 + t6204 + t6207 - t6498 - t6319 + t6322 - t6329 + t6333 + t6358 + t6116 + t6500 - t6504;
    (t6496, t6498, t6500, t6502, t6504, t6505)
}
