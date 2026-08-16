//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 462/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk462(t245: f64, t1580: f64, t21: f64, t2624: f64, t267: f64, t363: f64, t5: f64, t776: f64, t342: f64, t630: f64, t784: f64, t294: f64, t668: f64) -> (f64, f64, f64) {
    let t246 = 10000000.0_f64 <= t245;
    let t2635 = piecewise3(t246, 0.0_f64, t5 * t2624 * t21 / 4.0_f64 + t5 * t776 * t363 / 2.0_f64 + t5 * t267 * t1580 / 4.0_f64);
    let t2638 = t342 * t630 * t784 / 12.0_f64;
    let t2639 = t294 * t668;
    (t2635, t2638, t2639)
}
