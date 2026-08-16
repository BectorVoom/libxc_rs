//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1146/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1146(t31159: f64, t6932: f64, t1352: f64, t6943: f64, t6936: f64, t1332: f64, t8465: f64, t8467: f64, t1338: f64, t240: f64, t241: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31160 = t6932 * t31159;
    let t31162 = t6943 * t1352;
    let t31163 = t6936 * t31162;
    let t31165 = t1332 * t8465;
    let t31166 = t31165 * t8467;
    let t31169 = t1338 * t240 * t241;
    let t31170 = t1336 * t31169;
    (t31160, t31162, t31163, t31165, t31166, t31169, t31170)
}
