//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 699/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk699(t10447: f64, t7101: f64, t1091: f64, t25368: f64, t2881: f64, t3746: f64, t6360: f64, t28925: f64, t296: f64, t1508: f64, t835: f64, t6393: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29147 = t10447 * t7101;
    let t29150 = t25368 * t1091;
    let t29151 = t2881 * t29150;
    let t29154 = t6360 * t3746;
    let t29155 = t2881 * t29154;
    let t29158 = t296 * t28925;
    let t29162 = t835 * t1508 * t3746;
    let t29166 = t835 * t6393 * t1091;
    (t29147, t29150, t29151, t29154, t29155, t29158, t29162, t29166)
}
