//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 742/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk742<F: Float>(t2489: F, t7014: F, t1366: F, t2465: F, t2464: F, t587: F, t4167: F, t2487: F, t1415: F, t1428: F) -> (F, F, F, F) {
    let t7015 = t7014 * t2489;
    let t7017 = t2465 * t1366;
    let t7018 = t2464 * t7017;
    let t7019 = t587 * t7018;
    let t7021 = t2465 * t4167;
    let t7022 = t2464 * t7021;
    let t7023 = t2487 * t7022;
    let t7025 = t1415 * t1428;
    (t7015, t7019, t7023, t7025)
}
