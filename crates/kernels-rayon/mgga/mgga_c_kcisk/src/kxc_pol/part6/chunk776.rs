//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 776/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk776(t2507: f64, t5060: f64, t2399: f64, t4822: f64, t2456: f64, t4995: f64, t2449: f64, t2454: f64, t3934: f64, t649: f64, t164: f64, t2465: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17056 = t2507 * t5060;
    let t17057 = t17056 * sigma2;
    let t17078 = t2399 * t4822;
    let t17220 = t2456 * t4995;
    let t17222 = t2449 * t4995;
    let t17248 = t649 * t2454 * t3934;
    let t17276 = t164 * t2465;
    (t17056, t17057, t17078, t17220, t17222, t17248, t17276)
}
