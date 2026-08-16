//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1521/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1521(t12283: f64, t5259: f64, t5293: f64, t120: f64, t5286: f64, t5303: f64, t1340: f64, t16060: f64, t3789: f64, t5234: f64, t3798: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16239 = 7.0_f64 / 576.0_f64 * t12283 * t5259;
    let t16241 = 7.0_f64 / 2304.0_f64 * t12283 * t5293;
    let t16242 = t120 * t5286;
    let t16269 = 7.0_f64 / 576.0_f64 * t12283 * t5303;
    let t16278 = t16060 * t1340;
    let t16285 = t5234 * t3789;
    let t16288 = t5234 * t3798;
    (t16239, t16241, t16242, t16269, t16278, t16285, t16288)
}
