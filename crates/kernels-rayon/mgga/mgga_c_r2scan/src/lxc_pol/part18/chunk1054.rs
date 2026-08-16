//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1054/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1054(t37442: f64, t10659: f64, t10943: f64, t3428: f64, t3430: f64, t6818: f64, t260: f64, t6100: f64, t1102: f64, t1104: f64, t3314: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37443 = 0.45731474687362542471e-3_f64 * t37442;
    let t37444 = t10943 * t10659;
    let t37447 = t6818 * t3428 * t3430;
    let t37448 = 0.91462949374725084942e-3_f64 * t37447;
    let t37449 = t260 * t6100;
    let t37451 = t1102 * t37449 * t1104;
    let t37452 = 0.69557008413371175709e-2_f64 * t37451;
    let t37453 = t3314 * t875;
    (t37443, t37444, t37448, t37449, t37452, t37453)
}
