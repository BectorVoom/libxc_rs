//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 326/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk326(t1173: f64, t671: f64, t674: f64, t128: f64, t495: f64, t118: f64, t1986: f64) -> (f64, f64, f64, f64) {
    let t1993 = t671 * t1173;
    let t1994 = t1993 * t674;
    let t1995 = t128 * t495;
    let t1996 = t118 * t1995;
    let t1997 = t1986 * t1996;
    (t1993, t1994, t1995, t1997)
}
