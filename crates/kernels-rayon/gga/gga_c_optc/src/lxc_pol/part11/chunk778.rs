//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 778/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk778(t1256: f64, t127: f64, t2030: f64, t4631: f64, t2024: f64, t4649: f64, t4623: f64, t6879: f64, t4616: f64, t6941: f64, t2007: f64, t4620: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13190 = t127 * t1256;
    let t13202 = t2030 * t4631;
    let t13204 = t4649 * t2024;
    let t13209 = t4623 * t6879;
    let t13214 = t4623 * t127;
    let t13248 = t4649 * t127;
    let t13260 = t6941 * t4616;
    let t13262 = t2007 * t4620;
    (t13190, t13202, t13204, t13209, t13214, t13248, t13260, t13262)
}
