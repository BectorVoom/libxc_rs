//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 813/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk813(t21870: f64, t21897: f64, t332: f64, t113: f64, t10214: f64, t4917: f64, t10222: f64, t2639: f64, t4635: f64, t231: f64, t5299: f64, t10207: f64, t1526: f64, t18959: f64, t18977: f64, t2320: f64, t342: f64, t343: f64, t3806: f64, t5207: f64, t5213: f64, t5305: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21898 = t21870 + t21897;
    let t21899 = t21898 * t332;
    let t21900 = t21899 * t113;
    let t21911 = t10214 * t4917;
    let t21918 = t10222 * t4917;
    let t21922 = t2639 * t4635;
    let t21926 = t231 * t5299;
    let t21930 = t5207 + t5305 + t10207 - t18959 / 18.0_f64 - t18977 / 6.0_f64 - t1526 * t3806 * t21911 / 9.0_f64 - t1526 * t2320 * t5213 / 6.0_f64 + t1526 * t2320 * t21918 / 6.0_f64 - t1526 * t2320 * t21922 / 12.0_f64 - t342 * t343 * t21926 / 4.0_f64;
    (t21898, t21899, t21900, t21911, t21918, t21922, t21926, t21930)
}
