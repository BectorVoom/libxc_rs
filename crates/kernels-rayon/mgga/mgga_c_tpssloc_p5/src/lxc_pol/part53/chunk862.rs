//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 862/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk862(t645: f64, t8307: f64, t8513: f64, t31: f64, t607: f64, t641: f64, t79: f64, t12461: f64, t1388: f64, t2314: f64, t8327: f64, t4034: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31005 = t8307 * t645;
    let t31006 = t8513 * t31005;
    let t31011 = t8307 * t31;
    let t31013 = t8513 * t31011 * t607;
    let t31024 = t8513 * t79 * t641;
    let t31043 = t12461 * t1388;
    let t31054 = t2314 * t8327;
    let t31055 = 2.0_f64 * t31054;
    let t31056 = t4034 * t8327;
    (t31006, t31011, t31013, t31024, t31043, t31055, t31056)
}
