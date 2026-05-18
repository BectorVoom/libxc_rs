//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 764/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk764<F: Float>(t4121: F, t789: F, t2206: F, t4117: F, t796: F, t1329: F, t238: F, t242: F, t226: F, t4104: F, t2203: F, t2216: F, t3300: F, t3342: F, t4106: F, t4118: F) -> (F, F, F, F, F, F, F, F) {
    let t4122 = t789 * t4121;
    let t4126 = t2206 * t4117;
    let t4128 = t796 * t4121;
    let t4131 = t1329 * t1329;
    let t4133 = t238 * t242 * t4131;
    let t4135 = t226 * t4104;
    let t4137 = t238 * t242 * t4135;
    let t4139 = -F::new(0.9494625e0) * t4118 + F::new(0.1898925e1) * t4122 + t2203 - F::new(0.59793333333333333334e0) * t3300 + F::new(0.8969e0) * t4106 + F::new(0.15358125e0) * t4126 + F::new(0.3071625e0) * t4128 + t2216 - F::new(0.32862666666666666666e0) * t3342 + F::new(0.24647e0) * t4133 + F::new(0.24647e0) * t4137;
    (t4122, t4126, t4128, t4131, t4133, t4135, t4137, t4139)
}
