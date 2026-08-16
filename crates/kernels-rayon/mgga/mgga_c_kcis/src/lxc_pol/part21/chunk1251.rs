//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1251/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1251(t2173: f64, t93661: f64, t26714: f64, t7687: f64, t15573: f64, t26735: f64, t26717: f64, t26728: f64, t7690: f64, t93609: f64, t26823: f64, t7699: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93762 = t2173 * t93661;
    let t93764 = t7687 * t26714;
    let t93767 = t2173 * t15573 * t26735;
    let t93771 = t26728 * t26717;
    let t93773 = t7690 * t93609;
    let t93785 = t26823 * t7699;
    (t93762, t93764, t93767, t93771, t93773, t93785)
}
