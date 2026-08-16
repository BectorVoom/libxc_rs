//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 696/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk696<F: Float>(t1250: F, t3342: F, t508: F, t526: F, t235: F, t72: F, t3245: F, t774: F, t1248: F, t3234: F, t1213: F, t1222: F, t1244: F, t3239: F, t3241: F, t3244: F, t3247: F, t3251: F, t3258: F, t3263: F, t3268: F, t3271: F, t3277: F, t3329: F, t3334: F, t3340: F) -> (F, F, F, F, F, F) {
    let t3343 = t3342 * t1250;
    let t3346 = F::cast_from(1.0_f64) / t526 / t508;
    let t3347 = t235 * t3346;
    let t3348 = t3347 * t72;
    let t3350 = t3348 * t774 * t3245;
    let t3354 = t1248 * t774 * t3234;
    let t3357 = t3239 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t3241 + t3244 * t3247 / F::cast_from(16.0_f64) - t1213 * t3251 / F::cast_from(48.0_f64) + t3258 * t3263 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t3268 + t3271 * t3277 / F::cast_from(384.0_f64) - t1222 * t3329 / F::cast_from(3072.0_f64) - t1222 * t3334 / F::cast_from(3072.0_f64) + t3340 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t3343 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t1244 * t3350 - t1244 * t3354 / F::cast_from(768.0_f64);
    (t3343, t3346, t3348, t3350, t3354, t3357)
}
