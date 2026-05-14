//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1293/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1293<F: Float>(t1404: F, t2474: F, t9440: F, t11484: F, t23305: F, t2511: F, t4322: F, t7376: F, t11497: F, t23268: F, t2475: F, t31652: F, t31654: F, t31656: F, t31658: F, t31660: F, t31663: F, t31666: F, t31668: F) -> (F, F, F, F, F) {
    let t31671 = 4.0 * t2474 * t1404 * t9440;
    let t31673 = 0.19298375398431042081e3 * t23305 * t11484;
    let t31676 = 0.96491876992155210402e2 * t7376 * t4322 * t2511;
    let t31679 = 0.62071215503128080361e4 * t23268 * t11497 * t2475;
    let t31680 = t31652 + t31654 + t31656 + t31658 + t31660 + t31663 + t31666 - t31668 - t31671 - t31673 - t31676 - t31679;
    (t31671, t31673, t31676, t31679, t31680)
}
