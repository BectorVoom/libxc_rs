//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 987/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk987<F: Float>(t46653: F, t11400: F, t1424: F, t2299: F, t544: F, t11384: F, t11371: F, t2478: F, t6583: F, t18313: F, t3516: F, t2482: F, t31119: F) -> (F, F, F, F, F) {
    let t46654 = F::cast_from(0.11502877786176224903e1_f64) * t46653;
    let t46658 = F::cast_from(0.39722766613167140743e-1_f64) * t544 * t2299 * t11400 * t1424;
    let t46662 = F::cast_from(0.39722766613167140743e-1_f64) * t544 * t2299 * t11384 * t1424;
    let t46667 = t6583 * t11371 * t2478;
    let t46668 = F::cast_from(0.19171462976960374838e0_f64) * t46667;
    let t46669 = t18313 * t3516;
    let t46671 = t31119 * t46669 * t2482;
    (t46654, t46658, t46662, t46668, t46671)
}
