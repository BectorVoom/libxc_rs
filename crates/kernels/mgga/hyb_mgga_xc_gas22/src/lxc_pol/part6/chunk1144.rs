//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1144/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1144<F: Float>(t1819: F, t555: F, t8193: F, t7905: F, t8185: F, t19: F, t550: F, t8204: F, t8200: F, t1806: F, t2986: F, t2998: F, t6160: F, t7914: F, t7921: F, t7925: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23740 = t555 * t1819 * t8193;
    let t23743 = t555 * t8185 * t7905;
    let t23746 = t19 * t550 * t8204;
    let t23749 = t19 * t550 * t8200;
    let t23751 = t2986 * t1806;
    let t23756 = t555 * t6160 * t2998;
    let t23759 = t555 * t1819 * t7914;
    let t23762 = t555 * t1819 * t7921;
    let t23765 = t555 * t1819 * t7925;
    (t23740, t23743, t23746, t23749, t23751, t23756, t23759, t23762, t23765)
}
