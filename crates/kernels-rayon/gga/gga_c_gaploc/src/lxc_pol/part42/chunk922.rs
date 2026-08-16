//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 922/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk922(t2386: f64, t3529: f64, t544: f64, t6514: f64, t204: f64, t46362: f64, t587: f64, t2487: f64, t6711: f64, t4391: f64, t46254: f64, t6964: f64) -> (f64, f64, f64, f64) {
    let t46574 = 0.25025342966295298669e1_f64 * t544 * t6514 * t3529 * t2386;
    let t46577 = 0.18404604457881959845e2_f64 * t587 * t204 * t46362;
    let t46580 = 0.14953741122029092374e3_f64 * t2487 * t6711 * t46362;
    let t46583 = 0.42900587942220512003e1_f64 * t4391 * t6964 * t46254;
    (t46574, t46577, t46580, t46583)
}
