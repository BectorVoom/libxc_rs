//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1361/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1361(t15573: f64, t28178: f64, t7788: f64, t28183: f64, t7772: f64, t27014: f64, t27028: f64, t28132: f64, t28137: f64, t46849: f64, t47050: f64, t5329: f64, t92861: f64, t92872: f64, t92890: f64, t95949: f64, t95952: f64) -> f64 {
    let t97102 = 0.46336805555555555556e-3_f64 * t7788 * t15573 * t28178;
    let t97103 = t15573 * t28183;
    let t97105 = 0.23168402777777777778e-3_f64 * t7788 * t97103;
    let t97106 = t7772 * t97103;
    let t97125 = -t97102 - t97105 - 0.30918233506944444444e-4_f64 * t97106 - 0.69505208333333333334e-3_f64 * t27014 * t28132 - 0.13901041666666666667e-2_f64 * t27014 * t28137 - 0.69505208333333333334e-3_f64 * t7788 * t5329 * t27028 * t47050 - 0.23214722222222222222e-2_f64 * t95949 - 0.11607361111111111111e-2_f64 * t95952 - 0.23168402777777777778e-3_f64 * t92861 + 0.10317654320987654321e-2_f64 * t92872 - 0.3861400462962962963e-4_f64 * t92890 - 0.13901041666666666667e-2_f64 * t7788 * t5329 * t27028 * t46849;
    t97125
}
