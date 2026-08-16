//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 965/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk965(t2071: f64, t397: f64, t37458: f64, t554: f64, t538: f64, t1691: f64, t2035: f64, t8811: f64, t135: f64, t1696: f64, t1681: f64, t527: f64, t8832: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40053 = t397 * t2071;
    let t40055 = t37458 * t40053 * t554;
    let t40059 = t37458 * t40053 * t538;
    let t40067 = t2035 * t1691;
    let t40068 = t8811 * t40067;
    let t40069 = t1696 * t135;
    let t40078 = t37458 * t1681 * t554 * t538;
    let t40081 = t527 * t8832;
    (t40055, t40059, t40067, t40068, t40069, t40078, t40081)
}
