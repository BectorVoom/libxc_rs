//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 620/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk620(t4636: f64, t4722: f64, t4638: f64, t4642: f64, t4646: f64, t4650: f64, t4672: f64, t4674: f64, t4717: f64, t4719: f64, t4724: f64, t4728: f64, t4731: f64, t4734: f64) -> f64 {
    let t5380 = 0.68863333333333333333e0_f64 * t4636;
    let t5387 = 0.17365833333333333333e0_f64 * t4722;
    let t5392 = -0.17648625e1_f64 * t4672 + 0.3529725e1_f64 * t4674 + t5380 + 0.34431666666666666666e0_f64 * t4638 - 0.34431666666666666667e0_f64 * t4642 + 0.103295e1_f64 * t4646 - 0.516475e0_f64 * t4650 + 0.31558125e0_f64 * t4717 + 0.6311625e0_f64 * t4719 + t5387 + 0.13892666666666666667e0_f64 * t4724 - 0.34731666666666666667e-1_f64 * t4728 + 0.20839e0_f64 * t4731 - 0.104195e0_f64 * t4734;
    t5392
}
