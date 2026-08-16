//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1001/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1001<F: Float>(t1609: F, t1615: F, t2462: F, t2468: F, t2188: F, t286: F, t442: F, t7592: F, t291: F, t7875: F, t103: F, t332: F, t7877: F, t818: F) -> (F, F, F, F, F) {
    let t15430 = t1609 * t1615;
    let t15436 = t2462 * t2468;
    let t15473 = t7592 * t2188 * t286 * t442;
    let t15479 = t291 * t7875;
    let t15483 = t15479 * t332 * t818 * t7877 * t103;
    (t15430, t15436, t15473, t15479, t15483)
}
