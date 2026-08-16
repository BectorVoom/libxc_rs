//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 633/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk633(t22: f64, t3118: f64, t1210: f64, t3725: f64, t212: f64, t23: f64, t6: f64, t161: f64, t1048: f64, t9: f64, t7: f64, t171: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5744 = t22 * t3118;
    let t5794 = t3725 * t1210;
    let t5814 = 1.0_f64 / t23 / t212;
    let t5815 = t6 * t5814;
    let t5816 = t161 * t5815;
    let t5821 = 1.0_f64 / t9 / t1048;
    let t5822 = t7 * t5821;
    let t5823 = t171 * t5822;
    (t5744, t5794, t5814, t5816, t5821, t5823)
}
