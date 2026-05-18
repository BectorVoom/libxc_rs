//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 865/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk865<F: Float>(t2781: F, t7236: F, t5248: F, t5256: F, t5258: F, t5260: F, t7374: F, t7376: F, t7379: F, t7380: F, t7383: F, t7386: F, t7389: F, t7392: F, t7395: F, t7398: F, t7401: F, t7407: F) -> F {
    let t7409 = t7236 * t2781;
    let t7411 = -F::new(0.15996296296296296296e-1) * t7374 + F::new(0.26393888888888888889e0) * t7376 + t7379 - F::new(0.47988888888888888889e-1) * t7380 - F::new(0.39990740740740740742e-1) * t7383 + F::new(0.14396666666666666667e0) * t7386 - F::new(0.95977777777777777779e-1) * t7389 - F::new(0.23994444444444444445e-1) * t7392 - F::new(0.21595e0) * t7395 + F::new(0.28793333333333333334e0) * t7398 + F::new(0.71983333333333333334e-1) * t7401 - F::new(0.8888888888888888889e-2) * t5248 - F::new(0.14814814814814814815e-1) * t5256 + F::new(0.44444444444444444445e-2) * t5258 + F::new(0.14814814814814814815e-2) * t5260 - F::new(0.74074074074074074075e-2) * t7407 + F::new(0.57777777777777777777e-1) * t7409;
    t7411
}
