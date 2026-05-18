//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 892/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk892<F: Float>(t13578: F, t16710: F, t841: F, t23575: F, t3638: F, t13585: F, t5552: F, t2358: F, t35770: F, t3684: F, t7822: F, t2728: F, t5559: F) -> (F, F, F, F, F, F) {
    let t45983 = F::new(24.0) * t16710 * t13578 * t841;
    let t45988 = F::new(2.0) * t23575 * t3638;
    let t45990 = F::new(2.0) * t5552 * t13585;
    let t45992 = F::new(2.0) * t35770 * t2358;
    let t45993 = t7822 * t3684;
    let t45997 = F::new(6.0) * t5559 * t3638 * t2728;
    (t45983, t45988, t45990, t45992, t45993, t45997)
}
