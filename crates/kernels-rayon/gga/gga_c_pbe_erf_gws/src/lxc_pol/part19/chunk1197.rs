//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1197/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1197(t375: f64, t6125: f64, t336: f64, t9239: f64, t328: f64, t6552: f64, t1: f64, t6382: f64, t2052: f64, t3075: f64, t837: f64, t2306: f64, t3074: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20173 = 1.0_f64 / t6125 / t375;
    let t20842 = t9239 * t336;
    let t21296 = t6552 * t328;
    let t21518 = t6382 * t1;
    let t21884 = t2052 * t2052;
    let t21885 = 1.0_f64 / t21884;
    let t22334 = t3075 * t837;
    let t22336 = t3074 * t2306 * t22334;
    (t20173, t20842, t21296, t21518, t21885, t22336)
}
