//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1091/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1091(t411: f64, t7970: f64, t5651: f64, t14657: f64, t19516: f64, t19518: f64, t8902: f64, t14783: f64, t19523: f64, t19526: f64, t19533: f64, t19540: f64, t19544: f64, t8899: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20293 = t7970 * t411;
    let t20294 = t5651 * t20293;
    let t20301 = 52.61445_f64 * t14657 * t20293;
    let t20302 = 3.8973666666666666_f64 * t19516;
    let t20303 = 1.9486833333333333_f64 * t19518;
    let t20305 = 1.9486833333333333_f64 * t8902;
    let t20311 = -88.1424_f64 * t14783 * t20293 - t20301 + t20302 - t20303 - 0.97936_f64 * t8899 + t20305 + 5.87616_f64 * t19523 - 1.46904_f64 * t19526 + 2.0_f64 * t19533 - 2.0_f64 / 3.0_f64 * t19540 + 11.75232_f64 * t19544;
    (t20294, t20301, t20302, t20303, t20305, t20311)
}
