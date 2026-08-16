//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1394/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1394(t20374: f64, t7035: f64, t993: f64, t204: f64, t34459: f64, t7033: f64, t20229: f64, t6964: f64, t10513: f64, t20800: f64, t4391: f64, t10525: f64, t1564: f64, t539: f64) -> (f64, f64, f64, f64, f64) {
    let t34658 = t20374 * t993 * t7035;
    let t34659 = 0.38342925953920749676e0_f64 * t34658;
    let t34662 = 0.92023022289409799224e1_f64 * t7033 * t204 * t34459;
    let t34665 = 0.14300195980740170668e1_f64 * t20229 * t6964 * t34459;
    let t34668 = 0.57200783922960682671e1_f64 * t4391 * t20800 * t10513;
    let t34672 = 0.28600391961480341335e1_f64 * t10525 * t539 * t1564 * t10513;
    (t34659, t34662, t34665, t34668, t34672)
}
