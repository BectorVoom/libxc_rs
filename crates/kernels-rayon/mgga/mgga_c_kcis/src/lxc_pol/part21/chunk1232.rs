//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1232/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1232(t7772: f64, t92748: f64, t15573: f64, t26976: f64, t7788: f64, t1250: f64, t251: f64, t35547: f64, t27029: f64, t11000: f64, t1268: f64, t1241: f64, t209: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92749 = t7772 * t92748;
    let t92751 = t15573 * t26976;
    let t92752 = t7788 * t92751;
    let t92761 = t35547 * t251 * t1250;
    let t92785 = t7788 * t15573 * t27029;
    let t92787 = t11000 * t1268;
    let t92794 = t1241 * t209;
    (t92749, t92751, t92752, t92761, t92785, t92787, t92794)
}
