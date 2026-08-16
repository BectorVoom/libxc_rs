//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 595/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk595(t1336: f64, t140: f64, t4594: f64, t1801: f64, t4640: f64, t1800: f64, t1870: f64, t715: f64) -> (f64, f64, f64, f64, f64) {
    let t5054 = t140 * t1336 * t4594;
    let t5055 = t1801 * t4640;
    let t5056 = t1800 * t5055;
    let t5057 = t5054 * t5056;
    let t5060 = 1.0_f64 / t1870 / t715;
    (t5054, t5055, t5056, t5057, t5060)
}
