//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 750/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk750(t5155: f64, t960: f64, t10450: f64, t1835: f64, t10464: f64, t1919: f64, t5160: f64, t965: f64, t5163: f64, t1842: f64, t4726: f64, t10488: f64, t706: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11578 = t960 * t5155;
    let t11580 = t1835 * t10450;
    let t11583 = t1919 * t10464;
    let t11586 = t965 * t5160;
    let t11588 = t965 * t5163;
    let t11590 = t1842 * t10450;
    let t11593 = t4726 * t10464;
    let t11596 = t706 * t10488;
    (t11578, t11580, t11583, t11586, t11588, t11590, t11593, t11596)
}
