//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 281/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk281<F: Float>(t952: F, t957: F, t238: F, t353: F, t801: F, t343: F, t940: F, t242: F, t942: F, t953: F, t955: F) -> (F, F, F, F, F, F) {
    let t958 = t957 * t952;
    let t961 = t238 * t801 * t353;
    let t962 = F::cast_from(0.82156666666666666667e-1_f64) * t961;
    let t963 = t343 * t940;
    let t965 = t238 * t242 * t963;
    let t967 = F::new(0.1898925e1) * t953 - t955 + F::new(0.8969e0) * t942 + F::new(0.3071625e0) * t958 - t962 + F::new(0.24647e0) * t965;
    (t958, t961, t962, t963, t965, t967)
}
