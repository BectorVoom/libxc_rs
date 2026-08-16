//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 900/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk900(t3207: f64, t8042: f64, t1016: f64, t29096: f64, t10405: f64, t2482: f64, t9267: f64, t3338: f64, t4130: f64, t9272: f64, t12960: f64, t1537: f64) -> (f64, f64, f64, f64, f64) {
    let t41585 = t8042 * t3207;
    let t41586 = t29096 * t1016;
    let t41588 = t9267 * t10405 * t2482;
    let t41590 = t4130 * t3338;
    let t41592 = t9272 * t41590 * t2482;
    let t41594 = t1537 * t12960;
    (t41585, t41586, t41588, t41592, t41594)
}
