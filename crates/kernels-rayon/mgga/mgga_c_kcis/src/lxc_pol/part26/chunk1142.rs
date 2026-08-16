//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1142/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1142(t28876: f64, t449: f64, t446: f64, t448: f64, t6260: f64, t2233: f64, t1640: f64, t1884: f64, t5406: f64, t637: f64, t1881: f64, t7892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28877 = t449 * t28876;
    let t28878 = t446 * t28877;
    let t28880 = t448 * t6260;
    let t28881 = t2233 * t28880;
    let t28883 = t1884 * t1640;
    let t28884 = t2233 * t28883;
    let t28886 = t5406 * t637;
    let t28887 = t2233 * t28886;
    let t28889 = t1881 * t7892;
    (t28877, t28878, t28880, t28881, t28883, t28884, t28886, t28887, t28889)
}
