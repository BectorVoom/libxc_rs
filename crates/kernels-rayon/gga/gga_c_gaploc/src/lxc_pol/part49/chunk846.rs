//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 846/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk846(t1: f64, t10215: f64, t106: f64, t192: f64, t10496: f64, t540: f64, t1564: f64, t10600: f64, t1415: f64, t31590: f64, t493: f64, t26126: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34131 = t10215 * t1 * t106 * t192;
    let t34157 = t10496 * t540;
    let t34202 = t1564 * t10215;
    let t34264 = t1415 * t10600;
    let t34273 = t493 * t31590;
    let t34286 = t544 * t26126;
    (t34131, t34157, t34202, t34264, t34273, t34286)
}
