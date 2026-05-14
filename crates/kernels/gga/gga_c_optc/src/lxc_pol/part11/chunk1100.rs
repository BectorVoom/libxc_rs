//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1100/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1100<F: Float>(t37438: F, t22152: F, t22274: F, t22277: F, t22281: F, t22285: F, t3308: F, t37417: F, t37422: F, t38910: F, t4733: F, t47989: F, t48017: F, t37441: F, t28540: F, t1310: F, t16301: F, t22290: F, t22293: F, t23336: F, t28530: F, t48024: F, t48028: F, t48040: F, t55933: F, t606: F, t95: F) -> (F, F, F, F, F) {
    let t55994 = 72.0 * t37438;
    let t55995 = t22152 + 6.0 * t47989 + 3.0 * t37417 - 28.0 * t37422 + t22274 + t22277 + t22281 + t22285 + 0.31013857721884116596e-1 * t3308 * t38910 * t4733 - 14.0 / 3.0 * t48017 + t55994;
    let t55997 = 192.0 * t37441;
    let t56006 = 240.0 * t28540;
    let t56008 = t55997 + 2.0 * t1310 * t16301 + 2.0 * t48024 - 14.0 * t48028 + 140.0 / 3.0 * t28530 - t22290 + t22293 + t23336 + 0.77534644304710291488e-2 * t95 * t606 * t55933 + t56006 - 14.0 * t48040;
    (t55994, t55995, t55997, t56006, t56008)
}
