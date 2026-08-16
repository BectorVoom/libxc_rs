//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1059/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1059(t15762: f64, t233: f64, t2053: f64, t4574: f64, t564: f64, t1149: f64, t3299: f64, t1625: f64, t3465: f64, t222: f64, t3276: f64, t1056: f64, t3277: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15763 = t233 * t15762;
    let t15764 = t4574 * t2053;
    let t15765 = t564 * t15764;
    let t15766 = 3.0_f64 / 16.0_f64 * t15765;
    let t15767 = t3299 * t1149;
    let t15769 = t3465 * t1625;
    let t15770 = 3.0_f64 / 8.0_f64 * t15769;
    let t15772 = 1.0_f64 / t3276 / t222;
    let t15775 = t3277 * t1056;
    (t15763, t15766, t15767, t15770, t15772, t15775)
}
