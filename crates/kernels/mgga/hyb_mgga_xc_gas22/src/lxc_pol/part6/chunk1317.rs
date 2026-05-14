//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1317/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1317<F: Float>(t2849: F, t4540: F, t9691: F, t11524: F, t2824: F, t30771: F, t9501: F, t11315: F, t11475: F, t11536: F, t26194: F, t26421: F, t26429: F, t2834: F, t2910: F, t30787: F, t30790: F, t30818: F, t30822: F, t3680: F, t3688: F, t3757: F, t4565: F, t4571: F, t7800: F, t7811: F, t9575: F, t9696: F, t9747: F) -> (F, F, F, F) {
    let t30841 = t2849 * t4540 * t9691;
    let t30854 = t11524 * t2824;
    let t30860 = t9501 * t30771;
    let t30867 = -160.0 / 9.0 * t7800 * t11315 * t9696 - 64.0 / 27.0 * t3680 * t30841 - 32.0 / 9.0 * t2834 * t30818 + 64.0 / 27.0 * t3688 * t30822 - 4.0 * t4571 * t2910 + 2.0 * t9747 * t4565 - 5600.0 / 9.0 * t9575 * t30790 + 64.0 / 3.0 * t26429 * t30854 - 320.0 / 3.0 * t26421 * t11536 * t2824 + 704.0 / 81.0 * t3757 * t30860 + 64.0 / 9.0 * t26194 * t11475 + 32.0 / 9.0 * t7811 * t30787;
    (t30841, t30854, t30860, t30867)
}
