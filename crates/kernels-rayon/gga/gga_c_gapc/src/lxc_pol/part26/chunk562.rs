//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 562/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk562(t3254: f64, t3255: f64, t1061: f64, t2452: f64, t2456: f64, t3239: f64, t3219: f64, t3222: f64, t3228: f64, t3232: f64, t3236: f64, t3241: f64, t3245: f64, t3248: f64, t3251: f64) -> (f64, f64, f64) {
    let t3256 = t3254 * t3255;
    let t3258 = t1061 * t2452;
    let t3259 = t3239 * t2456;
    let t3260 = t3258 * t3259;
    let t3262 = -0.41036913933938047292e-5_f64 * t3219 - 0.13678971311312682431e-5_f64 * t3222 + 0.19948499828997661878e-6_f64 * t3228 - 0.41036913933938047292e-5_f64 * t3232 - 0.58714905980103539485e-5_f64 * t3236 + 0.58714905980103539485e-5_f64 * t3241 + 0.58714905980103539484e-6_f64 * t3245 - 0.1043951028326240932e-5_f64 * t3248 - 0.58714905980103539485e-5_f64 * t3251 - 0.1712518091086353235e-5_f64 * t3256 + 0.58714905980103539485e-5_f64 * t3260;
    (t3258, t3259, t3262)
}
