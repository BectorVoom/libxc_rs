//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1182/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1182<F: Float>(t2575: F, t568: F, t1058: F, t17245: F, t1535: F, t16923: F, t1816: F, t20350: F, t20351: F, t20352: F, t20354: F, t20357: F, t20359: F, t2536: F, t2718: F, t5082: F, t5162: F, t6758: F, t6806: F, t7181: F) -> (F, F) {
    let t20592 = t2575 * t568;
    let t20603 = t1058 * t17245;
    let t20610 = -F::new(18.0) * t1535 * t6806 * t7181 - F::new(3.0) * t1816 * t2536 * t7181 - F::new(6.0) * t20603 * t2536 * t5162 - F::new(18.0) * t2718 * t5082 * t6758 - t16923 - t20350 + t20351 - t20352 - t20354 - t20357 - t20359;
    (t20592, t20610)
}
