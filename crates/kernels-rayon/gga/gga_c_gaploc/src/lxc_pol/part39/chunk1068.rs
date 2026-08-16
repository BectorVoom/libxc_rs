//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1068/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1068(t8862: f64, t9780: f64, t1052: f64, t29646: f64, t10105: f64, t1960: f64, t3418: f64, t6553: f64, t10283: f64, t2497: f64, t13760: f64, t501: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44238 = 4.0_f64 * t8862 * t9780;
    let t44239 = t29646 * t1052;
    let t44242 = 2.0_f64 * t1960 * t1052 * t10105;
    let t44243 = t6553 * t3418;
    let t44245 = t10283 * t2497;
    let t46845 = t13760 * t501;
    (t44238, t44239, t44242, t44243, t44245, t46845)
}
