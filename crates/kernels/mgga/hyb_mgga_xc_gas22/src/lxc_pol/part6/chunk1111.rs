//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1111/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1111<F: Float>(t10888: F, t10890: F, t10893: F, t10898: F, t10913: F, t10915: F, t10922: F, t10924: F, t6969: F, t7082: F, t9008: F, t9161: F) -> F {
    let t10926 = F::new(0.142419375e1) * t10888 - F::new(0.1898925e1) * t10890 - F::new(0.9494625e0) * t10893 + F::new(0.1898925e1) * t10915 - t7082 + F::new(0.39862222222222222223e0) * t6969 + F::new(0.79724444444444444445e0) * t9008 - t9161 - F::new(0.29896666666666666667e0) * t10898 + F::new(0.8969e0) * t10913 - F::new(0.76790625e-1) * t10922 + F::new(0.3071625e0) * t10924;
    t10926
}
