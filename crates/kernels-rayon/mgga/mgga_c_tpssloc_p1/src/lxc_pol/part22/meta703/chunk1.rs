//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2291/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2291(t15572: f64, t15740: f64, t11697: f64, t18382: f64, t3577: f64, t1215: f64, t6224: f64, t1227: f64, t13969: f64, t18954: f64, t19067: f64, t1222: f64, t18297: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t66360 = t15740 * t15572;
    let t66363 = t3577 * t11697 * t18382;
    let t66388 = t6224 * t1215;
    let t66398 = t1227 * t13969 * t18954;
    let t66406 = t1227 * t13969 * t19067;
    let t66408 = t18297 * t1222;
    (t66360, t66363, t66388, t66398, t66406, t66408)
}
