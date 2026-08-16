//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 889/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk889(t167: f64, t2185: f64, t34853: f64, t23443: f64, t6626: f64, t23571: f64, t6699: f64, t12968: f64, t33191: f64, t925: f64, t9133: f64, t23470: f64, t6695: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35073 = t2185 * t167 * t34853;
    let t35076 = t23443 * t6626;
    let t35079 = t23571 * t6699;
    let t35080 = t12968 * t35079;
    let t35083 = t33191 * t925;
    let t35084 = t9133 * t35083;
    let t35087 = t23470 * t6695;
    (t35073, t35076, t35079, t35080, t35083, t35084, t35087)
}
