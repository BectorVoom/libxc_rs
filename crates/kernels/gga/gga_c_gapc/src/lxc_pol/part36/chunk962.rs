//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 962/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk962<F: Float>(t2456: F, t3728: F, t1062: F, t10335: F, t3643: F, t3734: F, t10286: F, t11270: F, t2923: F, t7108: F, t959: F, t3225: F) -> (F, F, F, F, F, F) {
    let t11640 = t3728 * t2456;
    let t11641 = t1062 * t11640;
    let t11643 = t3643 * t10335;
    let t11644 = t11643 * t3734;
    let t11646 = t11270 * t10286;
    let t11648 = t2923 * t959 * t7108;
    let t11649 = t11646 * t11648;
    let t11651 = t3225 * t3734;
    (t11640, t11641, t11644, t11648, t11649, t11651)
}
