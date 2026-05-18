//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 919/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk919<F: Float>(t557: F, t8184: F, t2988: F, t555: F, t1179: F, t6190: F, t6195: F, t1819: F, t2978: F, t3113: F, t668: F, t26: F) -> (F, F, F, F, F, F, F) {
    let t8185 = t8184 * t557;
    let t8187 = t555 * t8185 * t2988;
    let t8189 = t6190 * t1179;
    let t8193 = t6195 * t1179;
    let t8199 = t555 * t1819 * t2978 / F::new(96.0);
    let t8200 = t3113 * t668;
    let t8201 = t26 * t8200;
    (t8185, t8187, t8189, t8193, t8199, t8200, t8201)
}
