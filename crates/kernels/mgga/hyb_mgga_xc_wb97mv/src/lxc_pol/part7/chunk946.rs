//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 946/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk946<F: Float>(t2211: F, t3341: F, t2217: F, t3346: F, t790: F, t3352: F, t8908: F, t8967: F, t8969: F, t8973: F, t8976: F, t8979: F, t8983: F, t8987: F, t8990: F, t8960: F) -> (F, F, F, F) {
    let t8992 = t3341 * t2211;
    let t8994 = t2217 * t3346;
    let t8995 = t8994 * t790;
    let t8997 = t3352 * t2211;
    let t8999 = 0.40256666666666666667e0 * t8908 + 0.258925e1 * t8967 + 0.16504875e0 * t8969 - t8973 - t8976 + 0.248355e0 * t8979 + 0.49671e0 * t8983 + 0.248355e0 * t8987 - 0.258925e1 * t8990 - 0.1294625e1 * t8992 + 0.16504875e0 * t8995 + 0.82524375e-1 * t8997;
    let t9000 = t8960 + t8999;
    (t8992, t8995, t8997, t9000)
}
