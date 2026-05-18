//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1333/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1333<F: Float>(t24352: F, t2920: F, t35894: F, t10105: F, t3724: F, t10343: F, t11695: F, t12150: F, t12049: F, t12056: F, t30523: F, t8610: F) -> (F, F, F, F, F, F, F) {
    let t36040 = t2920 * t24352 * t35894;
    let t36042 = t10105 * t3724;
    let t36044 = t10343 * t11695;
    let t36091 = F::new(2.0) * t12150;
    let t36092 = F::new(2.0) * t12049;
    let t36093 = F::new(4.0) * t12056;
    let t36095 = F::new(6.0) * t30523 * t8610;
    (t36040, t36042, t36044, t36091, t36092, t36093, t36095)
}
