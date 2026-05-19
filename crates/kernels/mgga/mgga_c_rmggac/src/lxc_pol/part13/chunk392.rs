//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 392/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk392<F: Float>(t2415: F, t291: F, t2211: F, t570: F, t2217: F, t2220: F, t2223: F, t2226: F, t2348: F, t2351: F, t2354: F, t2357: F, t2359: F, t2361: F, t2363: F, t2365: F) -> (F, F, F) {
    let t2416 = t2415 * t291;
    let t2435 = t2211 * t570;
    let t2447 = -F::cast_from(0.19957069503106347607e-1_f64) * t2348 + F::cast_from(0.2993560425465952141e-1_f64) * t2351 + t2217 + F::cast_from(0.68186654135613354324e-2_f64) * t2354 - F::cast_from(0.90915538847484472432e-2_f64) * t2357 - t2220 - F::cast_from(0.66380770525302906695e-3_f64) * t2359 + F::cast_from(0.79656924630363488034e-3_f64) * t2361 + t2223 + F::cast_from(0.1814407727691612783e-3_f64) * t2363 - F::cast_from(0.21168090156402149135e-3_f64) * t2365 - t2226;
    (t2416, t2435, t2447)
}
