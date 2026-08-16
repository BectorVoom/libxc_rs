//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 534/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk534(t86: f64, t112: f64, t18: f64, t113: f64, t1577: f64, t3297: f64, t5: f64, t502: f64, t505: f64, t989: f64, t992: f64, t1022: f64, t1952: f64) -> (f64, f64, f64) {
    let t87 = 10000000.0_f64 <= t86;
    let t3307 = t112 * t18;
    let t3312 = piecewise3(t87, 0.0_f64, t5 * t3297 * t113 / 4.0_f64 + t5 * t989 * t505 / 4.0_f64 + t5 * t502 * t992 / 4.0_f64 - t5 * t3307 * t1577 / 2.0_f64);
    let t3313 = t1952 * t1022;
    (t3307, t3312, t3313)
}
