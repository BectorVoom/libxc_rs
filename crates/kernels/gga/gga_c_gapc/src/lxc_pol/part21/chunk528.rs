//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 528/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk528<F: Float>(t3258: F, t3259: F, t3219: F, t3222: F, t3228: F, t3232: F, t3236: F, t3241: F, t3245: F, t3248: F, t3251: F, t3256: F, t3182: F, t3184: F, t3190: F, t3194: F, t3199: F, t3202: F, t3204: F, t3207: F, t3210: F, t3213: F) -> (F,) {
    let t3260 = t3258 * t3259;
    let t3262 = -0.41036913933938047292e-5 * t3219 - 0.13678971311312682431e-5 * t3222 + 0.19948499828997661878e-6 * t3228 - 0.41036913933938047292e-5 * t3232 - 0.58714905980103539485e-5 * t3236 + 0.58714905980103539485e-5 * t3241 + 0.58714905980103539484e-6 * t3245 - 0.1043951028326240932e-5 * t3248 - 0.58714905980103539485e-5 * t3251 - 0.1712518091086353235e-5 * t3256 + 0.58714905980103539485e-5 * t3260;
    let t3263 = -0.46971924784082831588e-3 * t3182 + 0.28183154870449698953e-3 * t3184 - 0.28183154870449698953e-3 * t3190 - 0.93943849568165663176e-5 * t3194 + 0.16703216453219854913e-4 * t3199 + 0.28183154870449698953e-3 * t3202 + 0.37186107120732241674e-4 * t3204 - 0.28183154870449698953e-3 * t3207 - 0.1778266270470648716e-4 * t3210 + 0.41036913933938047292e-5 * t3213 + t3262;
    (t3263,)
}
