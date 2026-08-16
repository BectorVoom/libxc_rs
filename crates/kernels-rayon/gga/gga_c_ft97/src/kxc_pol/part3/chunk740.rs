//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 740/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk740(t3103: f64, t72: f64, t11280: f64, t1526: f64, t1527: f64, t15562: f64, t15567: f64, t15569: f64, t15576: f64, t15579: f64, t15584: f64, t2976: f64, t2988: f64, t3009: f64, t3109: f64, t342: f64, t343: f64, t7704: f64, t7707: f64, t7710: f64) -> f64 {
    let t15589 = t72 * t3103;
    let t15593 = t2976 + t3109 + t7704 - t7707 / 36.0_f64 - t7710 / 12.0_f64 - t15562 / 36.0_f64 - t15567 * t15569 / 9.0_f64 - t1526 * t1527 * t2988 / 12.0_f64 + t15567 * t15576 / 6.0_f64 - t1526 * t11280 * t15579 / 6.0_f64 - t15584 / 12.0_f64 - t1526 * t1527 * t3009 / 12.0_f64 - t342 * t343 * t15589 / 4.0_f64;
    t15593
}
