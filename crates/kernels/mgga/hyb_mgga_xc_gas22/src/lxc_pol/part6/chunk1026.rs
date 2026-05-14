//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1026/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1026<F: Float>(t143: F, t10330: F, t10262: F, t10267: F, t10270: F, t10275: F, t10278: F, t3188: F, t3196: F, t3201: F, t3206: F, t694: F, t708: F, t712: F, t716: F, t720: F, t724: F, t728: F, t732: F, t736: F) -> (F, F) {
    let t145 = 0.135e1 < t143;
    let t10331 = piecewise3(t145, t10330, 0.0);
    let t10348 = -t10262 * t708 / 80.0 + t3196 * t3188 / 320.0 + t10267 * t708 / 640.0 + t10270 * t708 / 1152.0 - t3201 * t3188 / 5760.0 - t10275 * t708 / 11520.0 - t10278 * t708 / 21504.0 + t3206 * t3188 / 129024.0 - t694 * t10331 / 18.0 + t712 * t10331 / 240.0 - t716 * t10331 / 4480.0 + t720 * t10331 / 103680.0 - t724 * t10331 / 2838528.0 + t728 * t10331 / 89456640.0 - t732 * t10331 / 0.31850496e10 + t736 * t10331 / 0.1263403008e12;
    (t10331, t10348)
}
