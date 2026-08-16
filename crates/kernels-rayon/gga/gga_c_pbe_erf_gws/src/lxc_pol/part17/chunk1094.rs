//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1094/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1094(t331: f64, t816: f64, t2080: f64, t2084: f64, t833: f64, t1195: f64, t2242: f64, t2409: f64, t6133: f64, t3959: f64, t326: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13942 = t816 * t331;
    let t13944 = t2080 * t2084 * t13942;
    let t13945 = t13944 * t833;
    let t13948 = 35.0_f64 / 432.0_f64 * t2242 * t1195;
    let t13949 = t2409 * t6133;
    let t13950 = t3959 * t13949;
    let t13952 = t326 * t837;
    (t13944, t13945, t13948, t13949, t13950, t13952)
}
