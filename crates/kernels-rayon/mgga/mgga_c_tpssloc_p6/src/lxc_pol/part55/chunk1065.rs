//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1065/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1065(t12524: f64, t8319: f64, t20173: f64, t1873: f64, t6534: f64, t3941: f64, t3938: f64, t8326: f64, t671: f64, t649: f64, t89: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31277 = 27.0_f64 * t12524 * t8319;
    let t31279 = 27.0_f64 * t20173 * t8319;
    let t31280 = t1873 * t6534;
    let t31282 = 54.0_f64 * t3941 * t31280;
    let t31283 = t3938 * t8326;
    let t31284 = 0.135e2_f64 * t31283;
    let t31285 = t8326 * t671;
    let t31286 = t3941 * t31285;
    let t31287 = 27.0_f64 * t31286;
    let t31537 = t649 * t1873;
    let t31540 = t89 * t6534;
    let t31717 = t88 * t6534;
    (t31277, t31279, t31280, t31282, t31284, t31285, t31287, t31537, t31540, t31717)
}
