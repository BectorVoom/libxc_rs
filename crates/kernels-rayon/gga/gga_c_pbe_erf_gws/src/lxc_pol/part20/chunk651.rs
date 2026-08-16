//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 651/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk651(t1780: f64, t3409: f64, t3413: f64, t3417: f64, t3419: f64, t3447: f64, t3449: f64, t3453: f64, t3458: f64, t3481: f64, t3490: f64, t3495: f64, t3496: f64) -> f64 {
    let t3592 = -t3409 + t3413 - t3417 - t3419 - t3447 + t3449 + t3453 + t3458 - t1780 + t3481 + t3490 + t3495 + t3496;
    t3592
}
