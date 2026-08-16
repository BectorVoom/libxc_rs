//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 793/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk793(t12797: f64, t1358: f64, t31591: f64, t4261: f64, t9074: f64, t2321: f64, t34600: f64, t12830: f64, t29874: f64, t12803: f64, t31586: f64, t34604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42673 = t1358 * t12797;
    let t42717 = t9074 * t4261 * t31591;
    let t42721 = t9074 * t34600 * t2321;
    let t42820 = t29874 * t12830;
    let t42825 = t1358 * t12803;
    let t42827 = t29874 * t12797;
    let t42846 = t29874 * t12803;
    let t42849 = t9074 * t4261 * t31586;
    let t42898 = t9074 * t34604 * t2321;
    (t42673, t42717, t42721, t42820, t42825, t42827, t42846, t42849, t42898)
}
