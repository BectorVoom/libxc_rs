//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 691/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk691(t13319: f64, t2268: f64, t13296: f64, t493: f64, t492: f64, t3531: f64, t894: f64, t12831: f64, t105: f64, t12821: f64, t13303: f64, t13306: f64, t13309: f64, t13312: f64, t13315: f64, t13316: f64) -> (f64, f64, f64, f64) {
    let t13321 = 0.28455006635676149599e-1_f64 * t2268 * t13319;
    let t13322 = t493 * t13296;
    let t13323 = t492 * t13322;
    let t13327 = t894 * t3531;
    let t13329 = 0.28455006635676149599e-1_f64 * t2268 * t13327;
    let t13330 = 0.142275033178380748e-1_f64 * t12831;
    let t13331 = t13303 + t13306 - t13309 + t13312 - t13315 + 0.56910013271352299198e-1_f64 * t2268 * t13316 + t13321 - 0.28455006635676149599e-1_f64 * t105 * t13323 - 0.47425011059460249332e-2_f64 * t12821 + t13329 - t13330;
    (t13322, t13323, t13327, t13331)
}
