//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 923/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk923(t42199: f64, t204: f64, t46254: f64, t587: f64, t2487: f64, t6711: f64, t46103: f64, t6710: f64, t13392: f64, t4391: f64, t549: f64, t2392: f64, t46499: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46584 = 0.23005755572352449806e1_f64 * t42199;
    let t46587 = 0.92023022289409799224e1_f64 * t587 * t204 * t46254;
    let t46590 = 0.43710935587469654631e2_f64 * t2487 * t6711 * t46254;
    let t46593 = 0.11502877786176224903e2_f64 * t6710 * t6711 * t46103;
    let t46595 = t4391 * t549 * t13392;
    let t46596 = 0.59584149919750711116e-1_f64 * t46595;
    let t46598 = 0.17875244975925213335e2_f64 * t46499 * t2392;
    (t46584, t46587, t46590, t46593, t46596, t46598)
}
