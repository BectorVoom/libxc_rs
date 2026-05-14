//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1065/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1065<F: Float>(t11009: F, t11011: F, t11016: F, t11020: F, t11023: F, t11027: F, t11031: F, t6815: F, t6817: F, t8958: F, t8973: F, t8976: F, t11085: F) -> (F,) {
    let t11095 = 0.82524375e-1 * t11009 + 0.16504875e0 * t11011 - t6815 + 0.27595e0 * t6817 + 0.5519e0 * t8958 - t8973 - t8976 - 0.16557e0 * t11016 + 0.49671e0 * t11020 - 0.16557e0 * t11023 + 0.248355e0 * t11027 + 0.248355e0 * t11031;
    let t11096 = t11085 + t11095;
    (t11096,)
}
