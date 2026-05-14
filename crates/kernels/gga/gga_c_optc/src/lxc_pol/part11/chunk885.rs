//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 885/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk885<F: Float>(t1037: F, t17423: F, t1459: F, t5170: F, t8688: F, t8686: F, t1460: F, t14852: F, t4144: F, t5187: F, t12366: F, t5190: F, t3020: F, t8582: F, t1221: F, t17348: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17425 = 1.0 * t1037 * t17423;
    let t17426 = t5170 * t1459;
    let t17427 = t17426 * t8688;
    let t17429 = 0.51725014705706168417e3 * t8686 * t17427;
    let t17431 = 3.0 * t14852 * t1460;
    let t17433 = 3.0 * t4144 * t5187;
    let t17435 = 0.48245472966453314466e2 * t12366 * t5190;
    let t17436 = t17426 * t3020;
    let t17438 = 0.96490945932906628932e2 * t8582 * t17436;
    let t17439 = t1221 * t17348;
    (t17425, t17426, t17427, t17429, t17431, t17433, t17435, t17436, t17438, t17439)
}
