//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1172/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1172<F: Float>(t676: F, t8861: F, t1285: F, t136: F, t3003: F, t1234: F, t6745: F, t8195: F, t8866: F, t549: F, t8871: F, t2015: F, t3296: F, t8626: F, t2003: F, t3304: F) -> (F, F, F, F, F, F, F, F) {
    let t25583 = t676 * t8861;
    let t25586 = t136 * t3003 * t1285;
    let t25588 = t1234 * t6745;
    let t25590 = t8195 * t8866;
    let t25593 = t136 * t549 * t8871;
    let t25595 = t2015 * t3296;
    let t25597 = t676 * t8626;
    let t25600 = t136 * t2003 * t3304;
    (t25583, t25586, t25588, t25590, t25593, t25595, t25597, t25600)
}
