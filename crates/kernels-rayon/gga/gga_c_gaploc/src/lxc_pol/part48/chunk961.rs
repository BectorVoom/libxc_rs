//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 961/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk961(t46254: f64, t6717: f64, t6914: f64, t2386: f64, t3516: f64, t544: f64, t6514: f64, t204: f64, t2476: f64, t46094: f64, t46115: f64, t587: f64) -> (f64, f64, f64, f64) {
    let t46257 = 0.62115540045351614476e2_f64 * t6914 * t6717 * t46254;
    let t46261 = 0.53625734927775640005e1_f64 * t544 * t6514 * t3516 * t2386;
    let t46264 = 0.92023022289409799224e1_f64 * t2476 * t204 * t46094;
    let t46267 = 0.18404604457881959845e2_f64 * t587 * t204 * t46115;
    (t46257, t46261, t46264, t46267)
}
