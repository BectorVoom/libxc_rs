//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 952/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk952(t1530: f64, t7540: f64, t25373: f64, t118480: f64, t22986: f64, t32814: f64, t86873: f64, t118472: f64, t1484: f64, t23270: f64, t112899: f64, t28267: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126197 = t7540 * t1530;
    let t126198 = t25373 * t126197;
    let t126226 = 0.15352717957250113407e0_f64 * t118480;
    let t126229 = 0.6579736267392905746e-1_f64 * t22986 * t86873 * t32814;
    let t126233 = 0.6579736267392905746e-1_f64 * t22986 * t23270 * t118472 * t1484;
    let t126240 = 0.6579736267392905746e-1_f64 * t22986 * t112899 * t28267;
    (t126197, t126198, t126226, t126229, t126233, t126240)
}
