//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 885/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk885(t2312: f64, t2317: f64, t3161: f64, t898: f64, t2328: f64, t2332: f64, t6122: f64, t890: f64, t6116: f64, t6196: f64, t6204: f64, t6207: f64, t6319: f64, t6322: f64, t6329: f64, t6333: f64, t6358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6496 = t2317 * t2312 * t3161;
    let t6498 = 0.51947577317044391277e2_f64 * t898 * t6496;
    let t6500 = 0.35089341735807877242e1_f64 * t2328 * t2332;
    let t6502 = t2317 * t6122 * t890;
    let t6504 = 0.35089341735807877242e1_f64 * t898 * t6502;
    let t6505 = t6196 + t6204 + t6207 - t6498 - t6319 + t6322 - t6329 + t6333 + t6358 + t6116 + t6500 - t6504;
    (t6496, t6498, t6500, t6502, t6504, t6505)
}
