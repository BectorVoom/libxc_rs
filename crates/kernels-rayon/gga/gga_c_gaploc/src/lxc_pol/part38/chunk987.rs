//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 987/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk987(t46653: f64, t11400: f64, t1424: f64, t2299: f64, t544: f64, t11384: f64, t11371: f64, t2478: f64, t6583: f64, t18313: f64, t3516: f64, t2482: f64, t31119: f64) -> (f64, f64, f64, f64, f64) {
    let t46654 = 0.11502877786176224903e1_f64 * t46653;
    let t46658 = 0.39722766613167140743e-1_f64 * t544 * t2299 * t11400 * t1424;
    let t46662 = 0.39722766613167140743e-1_f64 * t544 * t2299 * t11384 * t1424;
    let t46667 = t6583 * t11371 * t2478;
    let t46668 = 0.19171462976960374838e0_f64 * t46667;
    let t46669 = t18313 * t3516;
    let t46671 = t31119 * t46669 * t2482;
    (t46654, t46658, t46662, t46668, t46671)
}
