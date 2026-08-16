//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1234/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1234(t16225: f64, t3807: f64, t16224: f64, t12289: f64, t242: f64, t1336: f64, t16048: f64, t5248: f64, t5249: f64, t12283: f64, t5259: f64, t5293: f64) -> (f64, f64, f64, f64, f64) {
    let t16226 = t16225 * t3807;
    let t16227 = t16224 * t16226;
    let t16232 = t12289 * t242;
    let t16233 = t1336 * t16232;
    let t16235 = t5248 * t5249 * t16048;
    let t16239 = 7.0_f64 / 576.0_f64 * t12283 * t5259;
    let t16241 = 7.0_f64 / 2304.0_f64 * t12283 * t5293;
    (t16227, t16233, t16235, t16239, t16241)
}
