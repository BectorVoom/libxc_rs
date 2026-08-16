//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 897/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk897(t41660: f64, t2299: f64, t3516: f64, t1415: f64, t1646: f64, t12990: f64, t8072: f64, t44386: f64, t447: f64) -> (f64, f64, f64, f64) {
    let t46079 = 0.92023022289409799224e1_f64 * t41660;
    let t46088 = t2299 * t3516;
    let t46091 = 0.35750489951850426669e0_f64 * t1415 * t46088 * t1646;
    let t46093 = 0.71500979903700853338e0_f64 * t12990 * t8072;
    let t46094 = t44386 * t447;
    (t46079, t46091, t46093, t46094)
}
