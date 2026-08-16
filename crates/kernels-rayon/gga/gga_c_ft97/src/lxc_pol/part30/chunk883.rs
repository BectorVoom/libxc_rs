//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 883/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk883(t1466: f64, t1526: f64, t2: f64, t2320: f64, t342: f64, t34289: f64, t343: f64, t34301: f64, t36071: f64, t36075: f64, t36080: f64, t36086: f64, t7079: f64, t7084: f64, t7570: f64, t7571: f64) -> f64 {
    let t36091 = (-t36071 * t7571 / 6.0_f64 + t34289 + t1466 * t36075 / 18.0_f64 + t1466 * t7084 / 3.0_f64 - t7570 * t36080 / 6.0_f64 - t34301 - t1526 * t2320 * t7079 / 12.0_f64 - t342 * t343 * t36086 / 4.0_f64) * t2;
    t36091
}
