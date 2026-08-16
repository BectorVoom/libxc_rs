//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 807/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk807(t343: f64, t816: f64, t874: f64, t2251: f64, t916: f64, t2250: f64, t339: f64, t911: f64, t824: f64, t822: f64, t56: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6631 = t816 * t874 * t343;
    let t6636 = t2251 * t916;
    let t6637 = t2250 * t6636;
    let t6643 = t339 * t911;
    let t6644 = t824 * t6643;
    let t6645 = t822 * t6644;
    let t6658 = t56 * t931;
    (t6631, t6636, t6637, t6643, t6644, t6645, t6658)
}
