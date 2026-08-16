//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 916/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk916(t3859: f64, t684: f64, t10007: f64, t2568: f64, t737: f64, t3864: f64, t762: f64, t2608: f64, t3699: f64, t2486: f64, t3690: f64, t1091: f64, t2579: f64) -> (f64, f64, f64, f64, f64) {
    let t14171 = t3859 * t684;
    let t14172 = t10007 * t14171;
    let t14175 = t737 * t2568;
    let t14176 = t3864 * t684;
    let t14177 = t14175 * t14176;
    let t14182 = t737 * t762;
    let t14183 = t3699 * t2608;
    let t14184 = t14182 * t14183;
    let t14187 = t2486 * t762;
    let t14188 = t3690 * t2608;
    let t14189 = t14187 * t14188;
    let t14192 = t1091 * t2579;
    (t14172, t14177, t14184, t14189, t14192)
}
