//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 367/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk367(t86: f64, t112: f64, t113: f64, t1927: f64, t1934: f64, t5: f64, t502: f64, t505: f64, t342: f64, t511: f64, t630: f64, t142: f64, t358: f64) -> (f64, f64, f64) {
    let t87 = 10000000.0_f64 <= t86;
    let t1939 = piecewise3(t87, 0.0_f64, t5 * t1927 * t113 / 4.0_f64 + t5 * t502 * t505 / 2.0_f64 + t5 * t112 * t1934 / 4.0_f64);
    let t1942 = t342 * t630 * t511 / 12.0_f64;
    let t1943 = t142 * t358;
    (t1939, t1942, t1943)
}
