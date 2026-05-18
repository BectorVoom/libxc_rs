//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 768/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk768<F: Float>(t2258: F, t2263: F, t3300: F, t3342: F, t4106: F, t4118: F, t4122: F, t4126: F, t4128: F, t4133: F, t4137: F, t829: F) -> (F, F) {
    let t4166 = -F::new(0.17648625e1) * t4118 + F::new(0.3529725e1) * t4122 + t2258 - F::new(0.103295e1) * t3300 + F::new(0.1549425e1) * t4106 + F::new(0.31558125e0) * t4126 + F::new(0.6311625e0) * t4128 + t2263 - F::new(0.41678e0) * t3342 + F::new(0.312585e0) * t4133 + F::new(0.312585e0) * t4137;
    let t4167 = t4166 * t829;
    (t4166, t4167)
}
