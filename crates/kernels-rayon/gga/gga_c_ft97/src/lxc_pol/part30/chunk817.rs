//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 817/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk817(t342: f64, t630: f64, t7574: f64, t231: f64, t6260: f64, t1466: f64, t1526: f64, t2: f64, t2320: f64, t34284: f64, t34289: f64, t34291: f64, t34296: f64, t343: f64, t6335: f64, t6340: f64, t7570: f64, t7571: f64) -> (f64, f64, f64) {
    let t34301 = t342 * t630 * t7574 / 12.0_f64;
    let t34305 = t231 * t6260;
    let t34310 = (-t34284 * t7571 / 6.0_f64 + t34289 + t1466 * t34291 / 18.0_f64 + t1466 * t6340 / 3.0_f64 - t7570 * t34296 / 6.0_f64 - t34301 - t1526 * t2320 * t6335 / 12.0_f64 - t342 * t343 * t34305 / 4.0_f64) * t2;
    (t34301, t34305, t34310)
}
