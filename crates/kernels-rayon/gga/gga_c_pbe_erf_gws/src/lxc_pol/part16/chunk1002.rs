//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1002/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1002(t6680: f64, t8978: f64, t1134: f64, t814: f64, t858: f64, t3065: f64, t6678: f64, t328: f64, t6643: f64, t824: f64, t822: f64, t874: f64, t8884: f64) -> (f64, f64, f64, f64, f64) {
    let t8980 = t8978 * t6680 / 48.0_f64;
    let t8981 = t1134 * t814;
    let t8982 = t858 * t8981;
    let t8983 = t3065 * t8982;
    let t8985 = t6678 * t8983 / 96.0_f64;
    let t8986 = t6643 * t328;
    let t8987 = t824 * t8986;
    let t8988 = t822 * t8987;
    let t8989 = t8884 * t874;
    (t8980, t8983, t8985, t8988, t8989)
}
