//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1008/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1008(t1286: f64, t137311: f64, t144623: f64, t144633: f64, t144635: f64, t144641: f64, t144643: f64, t144645: f64, t1564: f64, t25533: f64, t25570: f64, t25574: f64, t25579: f64, t28: f64, t3051: f64, t32016: f64, t32355: f64, t34575: f64, t497: f64, t5501: f64, t7161: f64, t925: f64) -> f64 {
    let t144647 = t1286 * t28 * t34575 * t497 / 6.0_f64 - t1286 * t28 * t32355 * t25533 / 3.0_f64 + t144623 / 54.0_f64 - t32016 * t25574 / 18.0_f64 - t7161 * t3051 * t25579 / 9.0_f64 - t32016 * t25570 / 18.0_f64 - t144633 / 9.0_f64 - t144635 / 18.0_f64 - t5501 * t1564 * t137311 * t925 / 18.0_f64 + t144641 / 9.0_f64 - 4.0_f64 * t144643 - 2.0_f64 * t144645;
    t144647
}
