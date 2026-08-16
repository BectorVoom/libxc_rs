//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1052/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1052(t74281: f64, t74284: f64, t74290: f64, t74299: f64, t74302: f64, t74305: f64, t74309: f64, t74314: f64, t74319: f64, t76949: f64, t76950: f64, t76951: f64, t76952: f64, t76955: f64, t76957: f64, t76959: f64, t76965: f64) -> f64 {
    let t80081 = t76949 + t76950 + t76951 - t76952 - t76955 - 0.35038612185802734374e-6_f64 * t74281 - t74284 + t76957 - 0.87596530464506835935e-6_f64 * t74290 + t76959 - 0.10511583655740820312e-5_f64 * t74299 + 0.10511583655740820312e-5_f64 * t74302 - 0.10511583655740820312e-5_f64 * t74305 - 0.35038612185802734374e-6_f64 * t74309 + 0.52557918278704101561e-6_f64 * t74314 - t76965 - 0.87596530464506835932e-6_f64 * t74319;
    t80081
}
