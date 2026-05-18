//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 854/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk854<F: Float>(t2477: F, t944: F, t222: F, t343: F, t6007: F, t1885: F, t940: F) -> (F, F, F, F) {
    let t6951 = t944 * t2477;
    let t6966 = t222 * t6007 * t343;
    let t6967 = F::new(0.28842592592592592592e-1) * t6966;
    let t6969 = t222 * t1885 * t940;
    (t6951, t6966, t6967, t6969)
}
