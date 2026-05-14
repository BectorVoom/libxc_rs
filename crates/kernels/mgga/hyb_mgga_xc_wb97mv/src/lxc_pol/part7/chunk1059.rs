//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1059/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1059<F: Float>(t10995: F, t788: F, t4166: F, t6802: F, t790: F, t3346: F, t3352: F, t10963: F, t10978: F, t10985: F, t10987: F, t10990: F, t6762: F, t6840: F, t8908: F, t9090: F) -> (F, F, F, F, F) {
    let t10996 = t788 * t10995;
    let t11002 = t6802 * t4166;
    let t11003 = t11002 * t790;
    let t11005 = t3352 * t3346;
    let t11007 = 0.142419375e1 * t10985 - 0.1898925e1 * t10987 - 0.9494625e0 * t10990 + 0.1898925e1 * t10996 - t6840 + 0.39862222222222222223e0 * t6762 + 0.79724444444444444445e0 * t8908 - t9090 - 0.29896666666666666667e0 * t10963 + 0.8969e0 * t10978 - 0.76790625e-1 * t11003 + 0.3071625e0 * t11005;
    (t10996, t11002, t11003, t11005, t11007)
}
