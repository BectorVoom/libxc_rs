//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 922/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk922<F: Float>(t2386: F, t3529: F, t544: F, t6514: F, t204: F, t46362: F, t587: F, t2487: F, t6711: F, t4391: F, t46254: F, t6964: F) -> (F, F, F, F) {
    let t46574 = F::cast_from(0.25025342966295298669e1_f64) * t544 * t6514 * t3529 * t2386;
    let t46577 = F::cast_from(0.18404604457881959845e2_f64) * t587 * t204 * t46362;
    let t46580 = F::cast_from(0.14953741122029092374e3_f64) * t2487 * t6711 * t46362;
    let t46583 = F::cast_from(0.42900587942220512003e1_f64) * t4391 * t6964 * t46254;
    (t46574, t46577, t46580, t46583)
}
