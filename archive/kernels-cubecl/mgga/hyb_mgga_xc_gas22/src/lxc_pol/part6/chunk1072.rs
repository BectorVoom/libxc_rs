//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1072/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1072<F: Float>(t143: F, t10330: F, t10262: F, t10267: F, t10270: F, t10275: F, t10278: F, t3188: F, t3196: F, t3201: F, t3206: F, t694: F, t708: F, t712: F, t716: F, t720: F, t724: F, t728: F, t732: F, t736: F) -> (F, F) {
    let t145 = F::cast_from(0.135e1_f64) < t143;
    let t10331 = piecewise3::<F>(t145, t10330, F::cast_from(0.0_f64));
    let t10348 = -t10262 * t708 / F::cast_from(80.0_f64) + t3196 * t3188 / F::cast_from(320.0_f64) + t10267 * t708 / F::cast_from(640.0_f64) + t10270 * t708 / F::cast_from(1152.0_f64) - t3201 * t3188 / F::cast_from(5760.0_f64) - t10275 * t708 / F::cast_from(11520.0_f64) - t10278 * t708 / F::cast_from(21504.0_f64) + t3206 * t3188 / F::cast_from(129024.0_f64) - t694 * t10331 / F::cast_from(18.0_f64) + t712 * t10331 / F::cast_from(240.0_f64) - t716 * t10331 / F::cast_from(4480.0_f64) + t720 * t10331 / F::cast_from(103680.0_f64) - t724 * t10331 / F::cast_from(2838528.0_f64) + t728 * t10331 / F::cast_from(89456640.0_f64) - t732 * t10331 / F::cast_from(0.31850496e10_f64) + t736 * t10331 / F::cast_from(0.1263403008e12_f64);
    (t10331, t10348)
}
