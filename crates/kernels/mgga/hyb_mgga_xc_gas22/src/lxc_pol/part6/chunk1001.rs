//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1001/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1001<F: Float>(t436: F, t9314: F, t1514: F, t2628: F, t2707: F, t3639: F, t10: F, t3636: F, t1107: F, t1523: F, t221: F) -> (F, F, F, F, F, F) {
    let t9315 = t9314 * t436;
    let t9316 = t2628 * t1514;
    let t9319 = t3639 * t2707;
    let t9321 = t3636 * t10;
    let t9323 = F::new(0.36622894612013090108e-3) * t9321 * t1107;
    let t9324 = t1523 * t221;
    (t9315, t9316, t9319, t9321, t9323, t9324)
}
