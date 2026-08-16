//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1700/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1700(t16242: f64, t5248: f64, t5250: f64, t12240: f64, t5249: f64, t3856: f64, t12283: f64, t5303: f64, t1352: f64, t3851: f64, t1340: f64, t16060: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16257 = t5248 * t16242 * t5250;
    let t16261 = t5248 * t5249 * t12240;
    let t16265 = t5248 * t5249 * t3856;
    let t16269 = 7.0_f64 / 576.0_f64 * t12283 * t5303;
    let t16271 = t5248 * t16242 * t1352;
    let t16275 = t5248 * t5249 * t3851;
    let t16278 = t16060 * t1340;
    (t16257, t16261, t16265, t16269, t16271, t16275, t16278)
}
