//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1003/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1003<F: Float>(t9463: F, t9511: F, t9565: F, t9617: F, t9677: F, t9723: F, t9764: F, t9810: F, t500: F, t3918: F, t550: F, t19: F, t1181: F, t2994: F, t1230: F, t2971: F) -> (F, F, F, F, F, F) {
    let t9813 = t9463 + t9511 + t9565 + t9617 + t9677 + t9723 + t9764 + t9810;
    let t9814 = t500 * t9813;
    let t9824 = t550 * t3918;
    let t9825 = t19 * t9824;
    let t9827 = t1181 * t2994;
    let t9829 = t2971 * t1230;
    (t9813, t9814, t9824, t9825, t9827, t9829)
}
