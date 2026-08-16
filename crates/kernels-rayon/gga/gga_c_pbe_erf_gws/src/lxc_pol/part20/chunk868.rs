//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 868/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk868(t2206: f64, t3195: f64, t1114: f64, t6677: f64, t1134: f64, t814: f64, t858: f64, t3065: f64, t328: f64, t6643: f64, t824: f64, t874: f64, t8884: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8973 = 7.0_f64 / 72.0_f64 * t2206 * t3195;
    let t8978 = t1114 * t6677;
    let t8981 = t1134 * t814;
    let t8982 = t858 * t8981;
    let t8983 = t3065 * t8982;
    let t8986 = t6643 * t328;
    let t8987 = t824 * t8986;
    let t8989 = t8884 * t874;
    (t8973, t8978, t8981, t8983, t8987, t8989)
}
