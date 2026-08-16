//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 927/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk927(t11384: f64, t1424: f64, t2299: f64, t544: f64, t11371: f64, t2478: f64, t6583: f64, t18313: f64, t3516: f64, t2482: f64, t31119: f64, t2375: f64, t37579: f64) -> (f64, f64, f64, f64) {
    let t46662 = 0.39722766613167140743e-1_f64 * t544 * t2299 * t11384 * t1424;
    let t46667 = t6583 * t11371 * t2478;
    let t46668 = 0.19171462976960374838e0_f64 * t46667;
    let t46669 = t18313 * t3516;
    let t46671 = t31119 * t46669 * t2482;
    let t46672 = 0.23005755572352449806e1_f64 * t46671;
    let t46674 = 0.27805936629216998521e0_f64 * t37579 * t2375;
    (t46662, t46668, t46672, t46674)
}
