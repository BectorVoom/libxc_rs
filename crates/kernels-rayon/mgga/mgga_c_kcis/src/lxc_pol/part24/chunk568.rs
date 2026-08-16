//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 568/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk568(t2823: f64, t2862: f64, t3052: f64, t3174: f64, t430: f64, t4550: f64, t4558: f64, t4775: f64, t4779: f64, t4787: f64, t4790: f64, t4794: f64, t4798: f64, t4803: f64, t4808: f64, t4816: f64, t4821: f64, t4826: f64, t4926: f64, t5272: f64) -> f64 {
    let t5280 = -0.23214722222222222222e-2_f64 * t4550 + 0.19345601851851851852e-2_f64 * t4558 - 0.17411041666666666666e-2_f64 * t4775 + 0.77382407407407407407e-3_f64 * t2823 - 0.17411041666666666666e-2_f64 * t4779 + 0.11607361111111111111e-2_f64 * t4787 - 0.46429444444444444443e-2_f64 * t4790 - 0.11607361111111111111e-2_f64 * t4794 + 0.77382407407407407407e-3_f64 * t4798 - 0.11607361111111111111e-2_f64 * t4803 + 0.77382407407407407407e-3_f64 * t4808 - 0.11607361111111111111e-2_f64 * t2862 + t5272 * t430 + 0.11607361111111111111e-2_f64 * t3052 + 0.77382407407407407407e-3_f64 * t3174 - 0.38691203703703703703e-3_f64 * t4816 + 0.34822083333333333332e-2_f64 * t4821 - 0.11607361111111111111e-2_f64 * t4826 + 0.17411041666666666666e-2_f64 * t4926;
    t5280
}
