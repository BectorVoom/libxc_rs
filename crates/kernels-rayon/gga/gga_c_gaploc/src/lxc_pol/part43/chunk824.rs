//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 824/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk824(t2798: f64, t9588: f64, t10295: f64, t19933: f64, t24215: f64, t3366: f64, t3207: f64, t8042: f64, t1016: f64, t29096: f64, t12960: f64, t1537: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41575 = t2798 * t9588;
    let t41579 = 12.0_f64 * t19933 * t10295;
    let t41581 = 4.0_f64 * t24215 * t3366;
    let t41585 = t8042 * t3207;
    let t41586 = t29096 * t1016;
    let t41594 = t1537 * t12960;
    (t41575, t41579, t41581, t41585, t41586, t41594)
}
