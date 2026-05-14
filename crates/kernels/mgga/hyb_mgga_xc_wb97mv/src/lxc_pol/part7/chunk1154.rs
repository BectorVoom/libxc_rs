//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1154/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1154<F: Float>(t1020: F, t1036: F, t23606: F, t23608: F, t23611: F, t23614: F, t23617: F, t23622: F, t23624: F, t23626: F, t23628: F, t23631: F, t2663: F, t2666: F, t23864: F, t437: F) -> (F, F) {
    let t23924 = 1.0 * t1020 * (-0.21099166666666666667e1 * t23606 + 0.202552e2 * t23608 - 0.75019259259259259258e1 * t23611 + 0.6564185185185185185e1 * t23614 + 0.31003950617283950618e1 * t23617 + 0.68258333333333333335e-1 * t23622 - 0.10921333333333333333e1 * t23624 + 0.12134814814814814815e1 * t23626 + 0.10617962962962962963e1 * t23628 + 0.13388493827160493828e1 * t23631) * t1036;
    let t23925 = t2663 * t2663;
    let t23928 = t2666 * t2666;
    let t23932 = 0.24955700379505800916e5 * t437 / t23925 * t23864 / t23928;
    (t23924, t23932)
}
