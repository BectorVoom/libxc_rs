//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 565/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk565<F: Float>(t3254: F, t3255: F, t1061: F, t2452: F, t2456: F, t3239: F, t3219: F, t3222: F, t3228: F, t3232: F, t3236: F, t3241: F, t3245: F, t3248: F, t3251: F) -> (F, F, F) {
    let t3256 = t3254 * t3255;
    let t3258 = t1061 * t2452;
    let t3259 = t3239 * t2456;
    let t3260 = t3258 * t3259;
    let t3262 = -F::new(0.41036913933938047292e-5) * t3219 - F::new(0.13678971311312682431e-5) * t3222 + F::new(0.19948499828997661878e-6) * t3228 - F::new(0.41036913933938047292e-5) * t3232 - F::new(0.58714905980103539485e-5) * t3236 + F::new(0.58714905980103539485e-5) * t3241 + F::new(0.58714905980103539484e-6) * t3245 - F::new(0.1043951028326240932e-5) * t3248 - F::new(0.58714905980103539485e-5) * t3251 - F::new(0.1712518091086353235e-5) * t3256 + F::new(0.58714905980103539485e-5) * t3260;
    (t3258, t3259, t3262)
}
