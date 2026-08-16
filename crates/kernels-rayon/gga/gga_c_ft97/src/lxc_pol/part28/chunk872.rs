//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 872/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk872(t2112: f64, t34853: f64, t1369: f64, t28: f64, t1009: f64, t7318: f64, t1008: f64, t2035: f64, t1013: f64, t71: f64, t420: f64, t7195: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34854 = t2112 * t34853;
    let t34856 = t1369 * t28 * t34854;
    let t34857 = t7318 * t1009;
    let t34864 = t2035 * t7318 * t1008;
    let t34868 = t2035 * t7318 * t1013;
    let t34871 = t71 * t1008;
    let t34872 = t420 * t34871;
    let t34873 = t7195 * t34872;
    (t34854, t34856, t34857, t34864, t34868, t34871, t34872, t34873)
}
