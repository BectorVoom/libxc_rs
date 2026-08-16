//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 639/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk639(t1906: f64, t8392: f64, t1873: f64, t1882: f64, t24: f64, t7241: f64, t486: f64, t100: f64, t1843: f64, t376: f64, t89: f64, t7822: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8393 = t8392 * t1906;
    let t8409 = t1882 * t1873;
    let t8411 = t24 * t7241;
    let t8416 = t486 * t486;
    let t8417 = 1.0_f64 / t8416;
    let t8418 = t100 * t8417;
    let t8430 = t89 * t376 * t1843;
    let t8437 = 2.0_f64 / 9.0_f64 * t7822;
    (t8393, t8409, t8411, t8418, t8430, t8437)
}
