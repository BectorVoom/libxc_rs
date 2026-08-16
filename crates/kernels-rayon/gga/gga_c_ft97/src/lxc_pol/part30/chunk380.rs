//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 380/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk380(t292: f64, t1472: f64, t4094: f64, t4099: f64, t4104: f64, t6045: f64, t6057: f64, t6229: f64, t6233: f64, t6242: f64, t6243: f64, t6249: f64, t6251: f64, t6255: f64, t6256: f64) -> f64 {
    let t293 = 0.1e-59_f64 < t292;
    let t6260 = piecewise3(t293, 0.45306850413028723348e0_f64 * t4094 * t6229 - 0.22653425206514361674e0_f64 * t4099 * t6233 - 0.45306850413028723348e0_f64 * t4104 * t6229 + 0.22653425206514361674e0_f64 * t1472 * t6233 - 0.10001700163888888889e0_f64 * t6242 * t6045 * t6243 + 0.10001700163888888889e0_f64 * t6249 * t6251 - t6255 - 0.16669500273148148149e-1_f64 * t6256 * t6057, 0.0_f64);
    t6260
}
