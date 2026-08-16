//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 711/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk711(t1882: f64, t7098: f64, t28860: f64, t296: f64, t4167: f64, t6353: f64, t840: f64, t1508: f64, t2862: f64, t4162: f64, t7055: f64, t28845: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29340 = t1882 * t7098;
    let t29342 = t296 * t28860;
    let t29346 = t840 * t6353 * t4167;
    let t29350 = t2862 * t1508 * t4162;
    let t29354 = t1882 * t7055;
    let t29356 = t296 * t28845;
    (t29340, t29342, t29346, t29350, t29354, t29356)
}
