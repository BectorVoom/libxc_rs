//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 951/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk951<F: Float>(t1357: F, t2322: F, t1345: F, t2261: F, t8911: F, t6762: F, t6765: F, t6817: F, t6820: F, t6823: F, t6840: F, t6847: F, t8929: F, t8952: F, t8955: F, t8958: F) -> (F, F, F, F) {
    let t9077 = t1357 * t2322;
    let t9084 = t1345 * t2261;
    let t9090 = 0.59793333333333333334e0 * t8911;
    let t9097 = -t6847 + 0.54771111111111111111e0 * t6817 - 0.16431333333333333333e0 * t6820 - 0.16431333333333333333e0 * t6823 - t6840 - t9090 + 0.8969e0 * t8929 + 0.79724444444444444446e0 * t6762 - 0.29896666666666666667e0 * t6765 + 0.142419375e1 * t8952 - 0.76790625e-1 * t8955 + 0.27385555555555555555e0 * t8958;
    (t9077, t9084, t9090, t9097)
}
