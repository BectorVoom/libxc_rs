//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 806/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk806(t245: f64, t1178: f64, t20044: f64, t21: f64, t21780: f64, t267: f64, t4431: f64, t5: f64, t5186: f64, t920: f64, t1273: f64, t5478: f64, t332: f64) -> (f64, f64) {
    let t246 = 10000000.0_f64 <= t245;
    let t21794 = piecewise3(t246, 0.0_f64, t5 * t21780 * t21 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t5 * t5186 * t920 + 3.0_f64 / 4.0_f64 * t5 * t1178 * t4431 + t5 * t267 * t20044 / 4.0_f64);
    let t21800 = t5478 * t1273;
    let t21801 = t21800 * t332;
    (t21794, t21801)
}
