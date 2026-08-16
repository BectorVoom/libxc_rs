//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 771/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk771(t86: f64, t18: f64, t502: f64, t112: f64, t113: f64, t12068: f64, t1577: f64, t1927: f64, t1934: f64, t3297: f64, t3307: f64, t5: f64, t505: f64, t7742: f64, t989: f64, t992: f64) -> f64 {
    let t87 = 10000000.0_f64 <= t86;
    let t12081 = t502 * t18;
    let t12091 = piecewise3(t87, 0.0_f64, t5 * t12068 * t113 / 4.0_f64 + t5 * t3297 * t505 / 2.0_f64 + t5 * t989 * t1934 / 4.0_f64 + t5 * t1927 * t992 / 4.0_f64 - t5 * t12081 * t1577 - t5 * t112 * t1577 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t5 * t3307 * t7742);
    t12091
}
