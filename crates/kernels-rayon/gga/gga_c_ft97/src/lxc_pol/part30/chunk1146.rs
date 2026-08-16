//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1146/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1146(t1466: f64, t36048: f64, t681: f64, t142677: f64, t142688: f64, t1479: f64, t1506: f64, t153550: f64, t153553: f64, t153555: f64, t153558: f64, t153560: f64, t153567: f64, t153621: f64, t153664: f64, t193: f64, t2: f64, t26: f64, t29040: f64, t35799: f64, t4: f64, t6210: f64, t6391: f64, t7022: f64) -> f64 {
    let t153672 = t1466 * t681 * t36048;
    let t153674 = 8.0_f64 * t153550 + t6210 * t35799 + t153553 / 9.0_f64 + 4.0_f64 * t153555 + 4.0_f64 * t153558 + 4.0_f64 * t153560 + 2.0_f64 / 9.0_f64 * t142677 + t1466 * t193 * t29040 * t1506 / 3.0_f64 + t142688 - t153567 / 18.0_f64 + t1466 * t193 * t7022 * t6391 / 3.0_f64 + (t153621 + t153664) * t2 * t4 * t26 * t1479 / 6.0_f64 - t153672 / 9.0_f64;
    t153674
}
