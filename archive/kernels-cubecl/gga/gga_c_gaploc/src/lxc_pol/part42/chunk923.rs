//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 923/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk923<F: Float>(t42199: F, t204: F, t46254: F, t587: F, t2487: F, t6711: F, t46103: F, t6710: F, t13392: F, t4391: F, t549: F, t2392: F, t46499: F) -> (F, F, F, F, F, F) {
    let t46584 = F::cast_from(0.23005755572352449806e1_f64) * t42199;
    let t46587 = F::cast_from(0.92023022289409799224e1_f64) * t587 * t204 * t46254;
    let t46590 = F::cast_from(0.43710935587469654631e2_f64) * t2487 * t6711 * t46254;
    let t46593 = F::cast_from(0.11502877786176224903e2_f64) * t6710 * t6711 * t46103;
    let t46595 = t4391 * t549 * t13392;
    let t46596 = F::cast_from(0.59584149919750711116e-1_f64) * t46595;
    let t46598 = F::cast_from(0.17875244975925213335e2_f64) * t46499 * t2392;
    (t46584, t46587, t46590, t46593, t46596, t46598)
}
