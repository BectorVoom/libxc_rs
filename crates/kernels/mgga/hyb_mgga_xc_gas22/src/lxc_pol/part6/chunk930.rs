//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 930/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk930<F: Float>(t1371: F, t2291: F, t1359: F, t2252: F, t1351: F, t2250: F, t1363: F, t2289: F, t2183: F, t3353: F, t8651: F, t6530: F, t6533: F, t6614: F, t6616: F, t6619: F, t6622: F, t8648: F, t8654: F, t8656: F, t8659: F, t8661: F) -> (F, F, F, F, F, F, F) {
    let t8815 = t1371 * t2291;
    let t8818 = t1359 * t2252;
    let t8821 = t1351 * t2250;
    let t8824 = t1363 * t2289;
    let t8828 = 2.0 * t2183 * t3353;
    let t8830 = 0.59793333333333333334e0 * t8651;
    let t8840 = 0.8969e0 * t8648 - t8830 - 0.1898925e1 * t8654 - 0.9494625e0 * t8656 + 0.3071625e0 * t8659 + 0.15358125e0 * t8661 + 0.79724444444444444446e0 * t6530 - 0.29896666666666666667e0 * t6533 - t6614 + 0.54771111111111111111e0 * t6616 - 0.16431333333333333333e0 * t6619 - 0.16431333333333333333e0 * t6622;
    (t8815, t8818, t8821, t8824, t8828, t8830, t8840)
}
