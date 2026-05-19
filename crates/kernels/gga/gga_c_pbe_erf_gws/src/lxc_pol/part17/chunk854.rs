//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 854/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk854<F: Float>(t625: F, t7257: F, t11: F, t7205: F, t2704: F, t1416: F, t2672: F, t5022: F, t5042: F, t5052: F, t5054: F, t5056: F, t5083: F, t5085: F, t5087: F, t7233: F, t7237: F, t7239: F, t7249: F, t7252: F, t7255: F) -> (F, F, F, F, F) {
    let t7258 = t625 * t7257;
    let t7259 = t11 * t7258;
    let t7261 = t625 * t7205;
    let t7262 = t2704 * t7261;
    let t7264 = t2672 * t1416;
    let t7265 = t625 * t7264;
    let t7266 = t11 * t7265;
    let t7268 = F::cast_from(0.71983333333333333334e-1_f64) * t7233 - F::cast_from(0.8888888888888888889e-2_f64) * t5022 - F::cast_from(0.57777777777777777777e-1_f64) * t7237 - F::cast_from(0.74074074074074074075e-2_f64) * t7239 - F::cast_from(0.23994444444444444444e-1_f64) * t5042 - F::cast_from(0.14814814814814814815e-1_f64) * t5052 + F::cast_from(0.44444444444444444445e-2_f64) * t5054 + F::cast_from(0.14814814814814814815e-2_f64) * t5056 - F::cast_from(0.31992592592592592592e-1_f64) * t5083 + F::cast_from(0.11997222222222222222e-1_f64) * t5085 + F::cast_from(0.7998148148148148148e-2_f64) * t5087 + F::cast_from(0.14396666666666666667e0_f64) * t7249 + F::cast_from(0.95977777777777777779e-1_f64) * t7252 - F::cast_from(0.23994444444444444445e-1_f64) * t7255 - F::new(0.21595e0) * t7259 - F::cast_from(0.28793333333333333334e0_f64) * t7262 + F::cast_from(0.71983333333333333334e-1_f64) * t7266;
    (t7259, t7262, t7264, t7266, t7268)
}
