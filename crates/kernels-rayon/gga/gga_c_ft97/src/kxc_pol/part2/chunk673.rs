//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 673/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk673(t1882: f64, t2528: f64, t760: f64, t255: f64, t2576: f64, t2571: f64, t9895: f64, t2492: f64, t754: f64, t2610: f64, t8392: f64, t9698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10048 = t1882 * t2528;
    let t10050 = t760 * t760;
    let t10051 = 1.0_f64 / t10050;
    let t10052 = t255 * t10051;
    let t10062 = t1882 * t2576;
    let t10064 = t1882 * t2571;
    let t10079 = t9895 * t255;
    let t10085 = t2492 * t754;
    let t10090 = t8392 * t2610;
    let t10119 = 28.0_f64 / 27.0_f64 * t9698;
    (t10048, t10052, t10062, t10064, t10079, t10085, t10090, t10119)
}
