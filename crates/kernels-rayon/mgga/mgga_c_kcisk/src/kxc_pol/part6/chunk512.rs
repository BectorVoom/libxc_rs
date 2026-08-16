//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 512/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk512(t5814: f64, t6: f64, t161: f64, t1048: f64, t9: f64, t7: f64, t171: f64, t156: f64, t3122: f64, t2198: f64, t960: f64, t2201: f64, t965: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5815 = t6 * t5814;
    let t5816 = t161 * t5815;
    let t5821 = 1.0_f64 / t9 / t1048;
    let t5822 = t7 * t5821;
    let t5823 = t171 * t5822;
    let t5827 = t156 * t3122;
    let t5831 = t960 * t2198;
    let t5833 = t965 * t2201;
    (t5816, t5821, t5823, t5827, t5831, t5833)
}
