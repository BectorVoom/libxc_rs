//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1139/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1139<F: Float>(t11288: F, t1161: F, t1129: F, t4512: F, t11270: F, t2858: F, t11274: F, t11267: F, t11271: F, t11275: F, t11279: F, t11283: F, t2821: F, t2829: F, t2834: F, t2838: F, t3661: F, t3680: F, t3688: F, t3733: F, t7637: F) -> (F, F, F) {
    let t11289 = t1161 * t11288;
    let t11292 = t4512 * t1129;
    let t11293 = t1161 * t11292;
    let t11296 = t2858 * t11270;
    let t11299 = t2858 * t11274;
    let t11310 = -F::cast_from(56.0_f64) / F::cast_from(3.0_f64) * t7637 * t11267 - F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t3733 * t11271 + F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t3661 * t11275 + F::cast_from(400.0_f64) / F::cast_from(27.0_f64) * t3733 * t11279 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2829 * t11283 + F::cast_from(400.0_f64) / F::cast_from(27.0_f64) * t3661 * t11279 + F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t2821 * t11289 - F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t2829 * t11293 - F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t2821 * t11296 + F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t2829 * t11299 - F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t3680 * t11271 + F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t3688 * t11275 - F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t2834 * t11296 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t2838 * t11299;
    (t11289, t11293, t11310)
}
