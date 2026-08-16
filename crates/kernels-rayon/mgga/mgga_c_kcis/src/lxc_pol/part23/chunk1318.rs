//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1318/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1318(t1300: f64, t2132: f64, t2233: f64, t27325: f64, t28876: f64, t3707: f64, t3708: f64, t446: f64, t5407: f64, t8014: f64, t8130: f64, t8255: f64, t91885: f64, t91895: f64, t91901: f64, t92157: f64, t92379: f64, t97561: f64) -> f64 {
    let t99758 = -t91885 - t446 * t1300 * t28876 / 8.0_f64 + t97561 - t2233 * t3707 * t2132 / 16.0_f64 + t91895 - t91901 + t92379 - t446 * t3708 * t8255 / 16.0_f64 + t92157 - t446 * t5407 * t8014 / 8.0_f64 + t8130 * t27325 / 16.0_f64;
    t99758
}
