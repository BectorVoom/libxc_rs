//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 997/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk997(t46818: f64, t10430: f64, t9263: f64, t993: f64, t1429: f64, t1445: f64, t1450: f64, t42400: f64, t42416: f64, t42420: f64, t42422: f64, t42425: f64, t42428: f64, t44268: f64, t44269: f64, t44381: f64, t447: f64, t46778: f64, t46785: f64, t46788: f64, t46792: f64, t46793: f64, t46799: f64, t46806: f64, t46811: f64, t46815: f64, t475: f64, t549: f64, t574: f64) -> f64 {
    let t46819 = 0.14896037479937677779e-1_f64 * t46818;
    let t46821 = t9263 * t993 * t10430;
    let t46823 = -t46778 - 0.46011511144704899612e1_f64 * t574 * t1445 * t44268 * t475 + t46785 - t46788 - 0.63904876589867916128e-1_f64 * t42400 - t46792 - t46793 + 0.17041300423964777634e0_f64 * t42416 - 0.12780975317973583226e0_f64 * t42420 - 0.76685851907841499353e0_f64 * t42422 - 0.76685851907841499353e0_f64 * t42425 + 0.31952438294933958064e0_f64 * t42428 - t46799 - 0.23005755572352449806e1_f64 * t1450 * t1445 * t44381 * t447 - t46806 + 0.39722766613167140743e-1_f64 * t1429 * t549 * t44269 - 0.12780975317973583226e1_f64 * t46811 + 0.85206502119823888171e-1_f64 * t46815 + t46819 - 0.76685851907841499354e0_f64 * t46821;
    t46823
}
