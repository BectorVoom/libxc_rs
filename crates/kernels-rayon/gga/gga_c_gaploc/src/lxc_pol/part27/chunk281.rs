//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 281/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk281(t1112: f64, t1114: f64, t1116: f64, t1099: f64, t1101: f64, t1105: f64, t1108: f64, t14: f64, t344: f64, t389: f64, t31: f64, t4: f64, t98: f64) -> (f64, f64) {
    let t1118 = -0.44044444444444444445e-2_f64 * t1112 + 0.88088888888888888889e-2_f64 * t1114 + 0.55033333333333333333e-2_f64 * t1116;
    let t1121 = -t1099 * t1101 / 18.0_f64 - t1105 * t344 / 6.0_f64 + t389 * t1108 / 9.0_f64 + t14 * t1118 / 2.0_f64;
    let t1126 = 0.14764770444444444444e-2_f64 * t4 * t98 * t31;
    (t1121, t1126)
}
