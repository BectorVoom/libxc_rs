//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1206/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1206(t5393: f64, t2843: f64, t1091: f64, t1255: f64, t1901: f64, t21362: f64, t22194: f64, t2857: f64, t296: f64, t319: f64, t44528: f64, t446: f64, t5299: f64, t5309: f64, t72805: f64, t835: f64, t840: f64, t84940: f64, t84958: f64, t84983: f64, t84985: f64, t88735: f64, t88749: f64) -> (f64, f64) {
    let t91124 = t5393 * t5393;
    let t91125 = t2843 * t91124;
    let t91136 = -4.0_f64 * t446 * t840 * t2843 * t5299 * t5309 - 8.0_f64 / 3.0_f64 * t446 * t835 * t1255 * t21362 - 8.0_f64 / 3.0_f64 * t446 * t2857 * t319 * t88735 - 8.0_f64 / 9.0_f64 * t84940 + 8.0_f64 / 3.0_f64 * t1901 * t44528 * t22194 * t1091 - 4.0_f64 / 3.0_f64 * t84958 + 2.0_f64 * t446 * t296 * t91125 - 8.0_f64 / 9.0_f64 * t72805 + 8.0_f64 / 3.0_f64 * t446 * t835 * t319 * t88749 - 8.0_f64 / 9.0_f64 * t84983 - 4.0_f64 / 9.0_f64 * t84985;
    (t91125, t91136)
}
