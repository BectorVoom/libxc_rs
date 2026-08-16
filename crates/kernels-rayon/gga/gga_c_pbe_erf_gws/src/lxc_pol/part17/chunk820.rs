//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 820/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk820(t2118: f64, t6638: f64, t339: f64, t911: f64, t824: f64, t822: f64, t2157: f64, t6177: f64, t337: f64, t2121: f64, t2302: f64, t2323: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6639 = t2118 * t6638;
    let t6643 = t339 * t911;
    let t6644 = t824 * t6643;
    let t6645 = t822 * t6644;
    let t6646 = t6177 * t2157;
    let t6647 = t337 * t6646;
    let t6648 = t2121 * t6647;
    let t6656 = t2323 * t2302;
    (t6639, t6643, t6644, t6645, t6646, t6648, t6656)
}
