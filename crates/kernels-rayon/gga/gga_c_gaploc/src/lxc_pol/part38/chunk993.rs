//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 993/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk993(t37956: f64, t895: f64, t1445: f64, t36117: f64, t574: f64, t874: f64, t11167: f64, t2293: f64, t13426: f64, t18658: f64, t44470: f64, t597: f64) -> (f64, f64, f64, f64, f64) {
    let t46742 = 0.23833659967900284446e0_f64 * t895 * t37956;
    let t46754 = 0.46011511144704899612e1_f64 * t574 * t1445 * t36117 * t874;
    let t46758 = 0.46011511144704899612e1_f64 * t574 * t1445 * t11167 * t2293;
    let t46760 = 0.21450293971110256001e1_f64 * t18658 * t13426;
    let t46765 = 0.11502877786176224903e2_f64 * t597 * t1445 * t44470;
    (t46742, t46754, t46758, t46760, t46765)
}
