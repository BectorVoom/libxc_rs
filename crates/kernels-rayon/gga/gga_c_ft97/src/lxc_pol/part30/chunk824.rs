//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 824/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk824(t1154: f64, t7440: f64, t7515: f64, t33282: f64, t7512: f64, t1091: f64, t33293: f64, t33294: f64, t33292: f64, t1131: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35309 = t7440 * t1154;
    let t35310 = t7515 * t35309;
    let t35312 = t33282 * t7512 * t35310;
    let t35315 = t33293 * t33294 * t1091;
    let t35316 = t33292 * t35315;
    let t35318 = t7440 * t1131;
    (t35309, t35310, t35312, t35315, t35316, t35318)
}
