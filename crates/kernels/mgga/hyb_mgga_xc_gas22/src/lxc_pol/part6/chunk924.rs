//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 924/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk924<F: Float>(t2028: F, t8296: F, t8297: F, t1238: F, t6299: F, t3171: F, t2054: F, t3: F, t675: F, t2002: F, t3172: F, t3177: F) -> (F, F, F, F, F) {
    let t8299 = t8296 * t8297 * t2028;
    let t8302 = t6299 * t1238;
    let t8304 = t3171 * t8302 * t2028;
    let t8307 = t2054 * t3;
    let t8309 = t3171 * t8307 * t675;
    let t8313 = t3171 * t3172 * t2002;
    let t8317 = t3177 * t3172 * t2028;
    (t8299, t8304, t8309, t8313, t8317)
}
