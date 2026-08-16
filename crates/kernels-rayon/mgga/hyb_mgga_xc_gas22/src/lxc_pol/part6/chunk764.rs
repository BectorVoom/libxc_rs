//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 764/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk764(t4121: f64, t789: f64, t2206: f64, t4117: f64, t796: f64, t1329: f64, t238: f64, t242: f64, t226: f64, t4104: f64, t2203: f64, t2216: f64, t3300: f64, t3342: f64, t4106: f64, t4118: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4122 = t789 * t4121;
    let t4126 = t2206 * t4117;
    let t4128 = t796 * t4121;
    let t4131 = t1329 * t1329;
    let t4133 = t238 * t242 * t4131;
    let t4135 = t226 * t4104;
    let t4137 = t238 * t242 * t4135;
    let t4139 = -0.9494625e0_f64 * t4118 + 0.1898925e1_f64 * t4122 + t2203 - 0.59793333333333333334e0_f64 * t3300 + 0.8969e0_f64 * t4106 + 0.15358125e0_f64 * t4126 + 0.3071625e0_f64 * t4128 + t2216 - 0.32862666666666666666e0_f64 * t3342 + 0.24647e0_f64 * t4133 + 0.24647e0_f64 * t4137;
    (t4122, t4126, t4128, t4131, t4133, t4135, t4137, t4139)
}
