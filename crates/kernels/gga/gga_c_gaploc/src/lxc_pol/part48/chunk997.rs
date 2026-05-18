//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 997/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk997<F: Float>(t46818: F, t10430: F, t9263: F, t993: F, t1429: F, t1445: F, t1450: F, t42400: F, t42416: F, t42420: F, t42422: F, t42425: F, t42428: F, t44268: F, t44269: F, t44381: F, t447: F, t46778: F, t46785: F, t46788: F, t46792: F, t46793: F, t46799: F, t46806: F, t46811: F, t46815: F, t475: F, t549: F, t574: F) -> F {
    let t46819 = F::new(0.14896037479937677779e-1) * t46818;
    let t46821 = t9263 * t993 * t10430;
    let t46823 = -t46778 - F::new(0.46011511144704899612e1) * t574 * t1445 * t44268 * t475 + t46785 - t46788 - F::new(0.63904876589867916128e-1) * t42400 - t46792 - t46793 + F::new(0.17041300423964777634e0) * t42416 - F::new(0.12780975317973583226e0) * t42420 - F::new(0.76685851907841499353e0) * t42422 - F::new(0.76685851907841499353e0) * t42425 + F::new(0.31952438294933958064e0) * t42428 - t46799 - F::new(0.23005755572352449806e1) * t1450 * t1445 * t44381 * t447 - t46806 + F::new(0.39722766613167140743e-1) * t1429 * t549 * t44269 - F::new(0.12780975317973583226e1) * t46811 + F::new(0.85206502119823888171e-1) * t46815 + t46819 - F::new(0.76685851907841499354e0) * t46821;
    t46823
}
