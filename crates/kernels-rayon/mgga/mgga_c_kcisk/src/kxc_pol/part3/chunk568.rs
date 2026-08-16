//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 568/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk568(t1685: f64, t4761: f64, t4762: f64, t4636: f64, t4722: f64, t4638: f64, t4642: f64, t4646: f64, t4650: f64, t4672: f64, t4674: f64, t4717: f64, t4719: f64, t4724: f64, t4728: f64, t4731: f64, t4734: f64) -> (f64, f64) {
    let t4764 = t4761 * t4762 * t1685;
    let t4769 = 0.40256666666666666667e0_f64 * t4636;
    let t4776 = 0.137975e0_f64 * t4722;
    let t4781 = -0.1294625e1_f64 * t4672 + 0.258925e1_f64 * t4674 + t4769 + 0.20128333333333333334e0_f64 * t4638 - 0.20128333333333333333e0_f64 * t4642 + 0.60385e0_f64 * t4646 - 0.301925e0_f64 * t4650 + 0.82524375e-1_f64 * t4717 + 0.16504875e0_f64 * t4719 + t4776 + 0.11038e0_f64 * t4724 - 0.27595e-1_f64 * t4728 + 0.16557e0_f64 * t4731 - 0.82785e-1_f64 * t4734;
    (t4764, t4781)
}
