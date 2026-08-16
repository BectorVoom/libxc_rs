//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 506/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk506(t5477: f64, t782: f64, t4597: f64, t786: f64, t2020: f64, t695: f64, t1849: f64, t2019: f64, t785: f64, t657: f64, t2040: f64, t801: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5479 = 0.59969295720591057378e-2_f64 * t782 * t5477;
    let t5486 = t786 * t4597;
    let t5491 = t2020 * t695;
    let t5497 = t786 * t1849;
    let t5507 = 1.0_f64 / t2019 / t785;
    let t5508 = t657 * t5507;
    let t5531 = 1.0_f64 / t2040 / t801;
    (t5479, t5486, t5491, t5497, t5507, t5508, t5531)
}
