//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 893/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk893(t16841: f64, t16856: f64, t799: f64, t779: f64, t1342: f64, t4818: f64, t7672: f64, t7669: f64, t10416: f64, t4898: f64, t2418: f64, t7681: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16857 = t16841 + t16856;
    let t16858 = t16857 * t799;
    let t16860 = 1.0_f64 * t779 * t16858;
    let t16861 = t4818 * t1342;
    let t16862 = t16861 * t7672;
    let t16864 = 0.51725014705706168417e3_f64 * t7669 * t16862;
    let t16866 = 6.0_f64 * t10416 * t4898;
    let t16867 = t16861 * t2418;
    let t16869 = 0.96490945932906628932e2_f64 * t7681 * t16867;
    (t16857, t16858, t16860, t16861, t16862, t16864, t16866, t16867, t16869)
}
