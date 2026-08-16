//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 594/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk594(t1791: f64, t5043: f64, t1801: f64, t4648: f64, t1800: f64, t1799: f64, t1693: f64, t1792: f64, t4583: f64, t4794: f64, t4800: f64, t4806: f64, t4809: f64, t4812: f64, t4814: f64, t4819: f64, t4823: f64, t4827: f64, t4830: f64, t671: f64) -> (f64, f64, f64, f64, f64) {
    let t5044 = t5043 * t1791;
    let t5048 = t1801 * t4648;
    let t5049 = t1800 * t5048;
    let t5050 = t1799 * t5049;
    let t5052 = 0.33163888888888888888e-2_f64 * t4583 - 0.24872916666666666666e-2_f64 * t4800 + 0.16581944444444444444e-2_f64 * t4806 - t4809 - 0.33163888888888888888e-2_f64 * t4812 + 0.22109259259259259258e-2_f64 * t4814 - 0.49745833333333333332e-2_f64 * t4819 + 0.74498e-1_f64 * t4823 * t4827 - 0.386e0_f64 * t4830 * t1792 - 0.193e0_f64 * t1693 * t5044 + t4794 * t671 + 0.16581944444444444444e-2_f64 * t5050;
    (t5044, t5048, t5049, t5050, t5052)
}
