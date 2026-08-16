//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 997/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk997(t11981: f64, t2464: f64, t2465: f64, t2487: f64, t13782: f64, t7014: f64, t13791: f64, t1429: f64, t549: f64, t40116: f64, t1445: f64, t1450: f64, t447: f64, t46919: f64) -> (f64, f64, f64, f64, f64) {
    let t47883 = t2487 * t2464 * t2465 * t11981;
    let t47885 = t7014 * t13782;
    let t47892 = t1429 * t549 * t13791;
    let t47895 = 0.85206502119823888171e-1_f64 * t40116;
    let t47900 = 0.23005755572352449806e1_f64 * t1450 * t1445 * t46919 * t447;
    (t47883, t47885, t47892, t47895, t47900)
}
