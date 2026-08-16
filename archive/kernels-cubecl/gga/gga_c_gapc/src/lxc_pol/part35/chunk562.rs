//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 562/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk562<F: Float>(t3254: F, t3255: F, t1061: F, t2452: F, t2456: F, t3239: F, t3219: F, t3222: F, t3228: F, t3232: F, t3236: F, t3241: F, t3245: F, t3248: F, t3251: F) -> (F, F, F) {
    let t3256 = t3254 * t3255;
    let t3258 = t1061 * t2452;
    let t3259 = t3239 * t2456;
    let t3260 = t3258 * t3259;
    let t3262 = -F::cast_from(0.41036913933938047292e-5_f64) * t3219 - F::cast_from(0.13678971311312682431e-5_f64) * t3222 + F::cast_from(0.19948499828997661878e-6_f64) * t3228 - F::cast_from(0.41036913933938047292e-5_f64) * t3232 - F::cast_from(0.58714905980103539485e-5_f64) * t3236 + F::cast_from(0.58714905980103539485e-5_f64) * t3241 + F::cast_from(0.58714905980103539484e-6_f64) * t3245 - F::cast_from(0.1043951028326240932e-5_f64) * t3248 - F::cast_from(0.58714905980103539485e-5_f64) * t3251 - F::cast_from(0.1712518091086353235e-5_f64) * t3256 + F::cast_from(0.58714905980103539485e-5_f64) * t3260;
    (t3258, t3259, t3262)
}
