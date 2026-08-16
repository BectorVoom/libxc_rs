//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1014/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1014(t11986: f64, t1445: f64, t2293: f64, t574: f64, t13749: f64, t1564: f64, t1562: f64, t475: f64, t40546: f64, t12277: f64, t2728: f64, t47064: f64) -> (f64, f64, f64, f64, f64) {
    let t48225 = t574 * t1445 * t11986 * t2293;
    let t48227 = t1564 * t13749;
    let t48231 = 0.69017266717057349418e1_f64 * t1562 * t1445 * t48227 * t475;
    let t48233 = 0.38342925953920749677e0_f64 * t40546;
    let t48242 = t12277 * t2728;
    let t50808 = 4.0_f64 * t47064;
    (t48225, t48231, t48233, t48242, t50808)
}
