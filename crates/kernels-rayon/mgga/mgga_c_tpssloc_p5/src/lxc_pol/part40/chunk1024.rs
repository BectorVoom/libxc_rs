//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1024/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1024(t12418: f64, t820: f64, t1351: f64, t1799: f64, t12289: f64, t242: f64, t1336: f64, t12283: f64, t5259: f64, t5293: f64, t120: f64, t5286: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16224 = t12418 * t820;
    let t16225 = t1799 * t1351;
    let t16232 = t12289 * t242;
    let t16233 = t1336 * t16232;
    let t16239 = 7.0_f64 / 576.0_f64 * t12283 * t5259;
    let t16241 = 7.0_f64 / 2304.0_f64 * t12283 * t5293;
    let t16242 = t120 * t5286;
    (t16224, t16225, t16233, t16239, t16241, t16242)
}
