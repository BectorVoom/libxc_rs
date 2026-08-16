//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1248/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1248(t10216: f64, t10969: f64, t135: f64, t4608: f64, t973: f64, t10868: f64, t1539: f64, t248: f64, t1041: f64, t1009: f64, t4552: f64, t1011: f64) -> (f64, f64, f64, f64, f64) {
    let t14187 = t10969 * t10216;
    let t14192 = t135 * t4608;
    let t14194 = t973 * t14192 / 432.0_f64;
    let t14202 = t248 * t10868 * t1539;
    let t14203 = t1041 * t14202;
    let t14205 = t4552 * t1009;
    let t14206 = t14205 * t1011;
    (t14187, t14194, t14203, t14205, t14206)
}
