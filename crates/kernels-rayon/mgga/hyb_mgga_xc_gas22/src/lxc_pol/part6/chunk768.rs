//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 768/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk768(t2258: f64, t2263: f64, t3300: f64, t3342: f64, t4106: f64, t4118: f64, t4122: f64, t4126: f64, t4128: f64, t4133: f64, t4137: f64, t829: f64) -> (f64, f64) {
    let t4166 = -0.17648625e1_f64 * t4118 + 0.3529725e1_f64 * t4122 + t2258 - 0.103295e1_f64 * t3300 + 0.1549425e1_f64 * t4106 + 0.31558125e0_f64 * t4126 + 0.6311625e0_f64 * t4128 + t2263 - 0.41678e0_f64 * t3342 + 0.312585e0_f64 * t4133 + 0.312585e0_f64 * t4137;
    let t4167 = t4166 * t829;
    (t4166, t4167)
}
