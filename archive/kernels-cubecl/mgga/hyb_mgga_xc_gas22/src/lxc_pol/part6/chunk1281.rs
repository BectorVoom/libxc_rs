//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1281/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1281<F: Float>(t1816: F, t3806: F, t1874: F, t1877: F, t10087: F, t551: F, t3814: F, t545: F, t668: F, t1796: F, t9838: F, t1230: F, t2970: F, t2974: F, t7847: F) -> (F, F, F, F, F, F, F) {
    let t27719 = t3806 * t1816;
    let t27721 = t3806 * t1874;
    let t27723 = t3806 * t1877;
    let t27725 = t10087 * t551;
    let t27728 = t668 * t3814 * t545;
    let t27732 = t9838 * t1796;
    let t27741 = t2970 * t7847 * t1230 * t2974;
    (t27719, t27721, t27723, t27725, t27728, t27732, t27741)
}
