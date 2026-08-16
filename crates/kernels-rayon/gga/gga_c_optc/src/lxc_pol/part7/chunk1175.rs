//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1175/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1175(t2568: f64, t2360: f64, t7239: f64, t7234: f64, t7308: f64, t190: f64, t2336: f64, t2350: f64, t2333: f64, t2253: f64, t7183: f64, t1000: f64, t23610: f64, t2364: f64, t23951: f64, t24354: f64, t2544: f64, t2551: f64, t2566: f64, t2722: f64, t277: f64, t2822: f64, t3980: f64, t4038: f64, t4039: f64, t7246: f64, t7285: f64, t8393: f64, t914: f64, t95: f64, t999: f64) -> f64 {
    let t24356 = t2568 * t2568;
    let t24357 = 1.0_f64 / t24356;
    let t24371 = t2360 * t7239;
    let t24373 = t7308 * t7234;
    let t24376 = t2350 * t190 * t2336;
    let t24377 = t2333 * t24376;
    let t24379 = t7183 * t2253;
    let t24385 = 0.31013857721884116596e-1_f64 * t3980 * t2566 * t8393 * t2822 - 0.15506928860942058298e-1_f64 * t95 * t277 * t24354 * t24357 - 4.0_f64 / 3.0_f64 * t4038 * t2722 * t4039 * t23951 - 16.0_f64 / 9.0_f64 * t2364 * t7246 - 4.0_f64 * t999 * t914 * t1000 * t23610 + 2.0_f64 / 3.0_f64 * t24371 + 80000.0_f64 / 81.0_f64 * t24373 - 80000.0_f64 / 243.0_f64 * t24377 + 200.0_f64 / 27.0_f64 * t24379 + 88.0_f64 / 9.0_f64 * t7285 * t2544 + 352.0_f64 / 27.0_f64 * t7285 * t2551;
    t24385
}
