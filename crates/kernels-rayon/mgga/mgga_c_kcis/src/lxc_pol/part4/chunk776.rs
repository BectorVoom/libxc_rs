//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 776/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk776(t4621: f64, t945: f64, t4714: f64, t2919: f64, t2955: f64, t2967: f64, t2968: f64, t4612: f64, t4615: f64, t4618: f64, t4623: f64, t4658: f64, t4660: f64, t4701: f64, t4703: f64, t4706: f64, t4709: f64, t4712: f64) -> (f64, f64, f64) {
    let t4715 = t945 * t4621;
    let t4716 = t4714 * t4715;
    let t4718 = -0.9494625e0_f64 * t4658 + 0.1898925e1_f64 * t4660 + t2955 + 0.99655555555555555557e-1_f64 * t2919 + 0.99655555555555555557e-1_f64 * t4612 - 0.19931111111111111111e0_f64 * t4615 + 0.59793333333333333334e0_f64 * t4618 - 0.59793333333333333334e0_f64 * t4623 + 0.15358125e0_f64 * t4701 + 0.3071625e0_f64 * t4703 + t2967 + 0.54771111111111111111e-1_f64 * t2968 + 0.54771111111111111111e-1_f64 * t4706 - 0.27385555555555555556e-1_f64 * t4709 + 0.16431333333333333333e0_f64 * t4712 - 0.16431333333333333333e0_f64 * t4716;
    (t4715, t4716, t4718)
}
