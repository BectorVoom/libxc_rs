//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 948/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk948<F: Float>(t12366: F, t5190: F, t17426: F, t3020: F, t8582: F, t1221: F, t17348: F, t914: F, t17336: F, t8426: F, t4305: F, t5308: F) -> (F, F, F, F, F, F, F, F) {
    let t17435 = F::new(0.48245472966453314466e2) * t12366 * t5190;
    let t17436 = t17426 * t3020;
    let t17438 = F::new(0.96490945932906628932e2) * t8582 * t17436;
    let t17439 = t1221 * t17348;
    let t17440 = t914 * t17439;
    let t17442 = t8426 * t17336;
    let t17443 = t914 * t17442;
    let t17447 = F::new(0.51947267698127589899e2) * t4305 * t5308;
    (t17435, t17436, t17438, t17439, t17440, t17442, t17443, t17447)
}
