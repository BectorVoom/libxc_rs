//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 971/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk971(t1165: f64, t14373: f64, t15758: f64, t530: f64, t14368: f64, t1532: f64, t322: f64, t4919: f64, t3382: f64, t5209: f64, t5213: f64, t3431: f64, t5281: f64) -> (f64, f64, f64, f64, f64) {
    let t15761 = t14373 * t1165 * t530 * t15758;
    let t15774 = t14368 * t1165 * t1532 * t4919 * t322;
    let t15776 = t3382 * t5209;
    let t15787 = t3382 * t5213;
    let t15789 = t3431 * t5281;
    (t15761, t15774, t15776, t15787, t15789)
}
