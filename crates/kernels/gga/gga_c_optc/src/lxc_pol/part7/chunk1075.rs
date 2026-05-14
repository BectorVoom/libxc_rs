//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1075/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1075<F: Float>(t2566: F, t2568: F, t2360: F, t7239: F, t7234: F, t7308: F, t190: F, t2336: F, t2350: F, t2333: F, t2253: F, t7183: F, t1000: F, t23610: F, t2364: F, t23951: F, t2544: F, t2551: F, t2722: F, t277: F, t2822: F, t3980: F, t4038: F, t4039: F, t7246: F, t7285: F, t8393: F, t914: F, t95: F, t999: F) -> (F,) {
    let t24354 = t2566 * t2566;
    let t24356 = t2568 * t2568;
    let t24357 = 1.0 / t24356;
    let t24371 = t2360 * t7239;
    let t24373 = t7308 * t7234;
    let t24376 = t2350 * t190 * t2336;
    let t24377 = t2333 * t24376;
    let t24379 = t7183 * t2253;
    let t24385 = 0.31013857721884116596e-1 * t3980 * t2566 * t8393 * t2822 - 0.15506928860942058298e-1 * t95 * t277 * t24354 * t24357 - 4.0 / 3.0 * t4038 * t2722 * t4039 * t23951 - 16.0 / 9.0 * t2364 * t7246 - 4.0 * t999 * t914 * t1000 * t23610 + 2.0 / 3.0 * t24371 + 80000.0 / 81.0 * t24373 - 80000.0 / 243.0 * t24377 + 200.0 / 27.0 * t24379 + 88.0 / 9.0 * t7285 * t2544 + 352.0 / 27.0 * t7285 * t2551;
    (t24385,)
}
