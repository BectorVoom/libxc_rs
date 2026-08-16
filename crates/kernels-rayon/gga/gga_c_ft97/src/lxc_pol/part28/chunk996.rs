//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 996/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk996(t33196: f64, t8392: f64, t7350: f64, t8232: f64, t1882: f64, t33180: f64, t33163: f64, t33092: f64, t160: f64, t32869: f64, t33052: f64, t33171: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t140275 = t8392 * t33196;
    let t140278 = 8.0_f64 / 27.0_f64 * t8232 * t7350;
    let t140288 = t1882 * t33180;
    let t140290 = t1882 * t33163;
    let t140325 = t1882 * t33092;
    let t140338 = t160 * t32869;
    let t140364 = t1882 * t33052;
    let t140370 = t1882 * t33171;
    (t140275, t140278, t140288, t140290, t140325, t140338, t140364, t140370)
}
