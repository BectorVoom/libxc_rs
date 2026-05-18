//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 927/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk927<F: Float>(t11384: F, t1424: F, t2299: F, t544: F, t11371: F, t2478: F, t6583: F, t18313: F, t3516: F, t2482: F, t31119: F, t2375: F, t37579: F) -> (F, F, F, F) {
    let t46662 = F::new(0.39722766613167140743e-1) * t544 * t2299 * t11384 * t1424;
    let t46667 = t6583 * t11371 * t2478;
    let t46668 = F::new(0.19171462976960374838e0) * t46667;
    let t46669 = t18313 * t3516;
    let t46671 = t31119 * t46669 * t2482;
    let t46672 = F::new(0.23005755572352449806e1) * t46671;
    let t46674 = F::new(0.27805936629216998521e0) * t37579 * t2375;
    (t46662, t46668, t46672, t46674)
}
