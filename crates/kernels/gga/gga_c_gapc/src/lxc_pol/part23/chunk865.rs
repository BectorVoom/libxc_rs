//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 865/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk865<F: Float>(t1086: F, t2628: F, t10046: F, t2233: F, t2982: F, t3387: F, t3138: F, t3363: F, t3330: F, t818: F, t959: F, t3329: F) -> (F, F, F, F, F, F) {
    let t10047 = t1086 * t2628;
    let t10048 = t10046 * t10047;
    let t10050 = t2982 * t2233;
    let t10051 = t3387 * t10050;
    let t10053 = t3363 * t3138;
    let t10054 = t10053 * t3330;
    let t10057 = M_PI * t818 * t959;
    let t10058 = t10057 * t3329;
    (t10047, t10048, t10051, t10054, t10057, t10058)
}
