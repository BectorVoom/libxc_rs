//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 950/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk950<F: Float>(t1336: F, t6585: F, t2195: F, t6601: F, t1329: F, t1885: F, t222: F) -> (F, F, F, F, F) {
    let t8669 = t6585 * t1336;
    let t8670 = t8669 * t2195;
    let t8672 = t6601 * t1336;
    let t8673 = t8672 * t2195;
    let t8676 = t222 * t1885 * t1329;
    (t8669, t8670, t8672, t8673, t8676)
}
