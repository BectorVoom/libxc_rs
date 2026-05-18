//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1267/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1267<F: Float>(t1026: F, t1046: F, t1266: F, t11424: F, t3665: F, t561: F, t5325: F, t5977: F, t5979: F, t116: F, t33965: F, t11402: F) -> (F, F, F, F, F) {
    let t35077 = t1266 * t1026 * t1046;
    let t35080 = t561 * t11424 * t3665;
    let t35083 = t5977 * t5325 * t5979;
    let t35085 = t116 * t33965;
    let t35086 = t35085 * t11402;
    (t35077, t35080, t35083, t35085, t35086)
}
