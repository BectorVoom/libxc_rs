//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1261/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1261<F: Float>(t11018: F, t238: F, t800: F, t1323: F, t242: F, t8927: F, t26385: F, t26389: F, t26392: F, t26411: F, t26414: F, t26417: F, t30886: F, t30902: F, t30904: F, t30907: F) -> (F, F, F) {
    let t30910 = t238 * t800 * t11018;
    let t30914 = t238 * t242 * t1323 * t8927;
    let t30916 = 0.3071625e0 * t30886 - 0.32862666666666666666e0 * t26385 - 0.65725333333333333332e0 * t26389 - 0.32862666666666666666e0 * t26392 + 0.10954222222222222222e1 * t26411 + 0.10954222222222222222e1 * t26414 - 0.14605629629629629629e1 * t26417 + 0.1898925e1 * t30902 + 0.3071625e0 * t30904 + 0.27385555555555555555e0 * t30907 - 0.65725333333333333333e0 * t30910 + 0.49294e0 * t30914;
    (t30910, t30914, t30916)
}
