//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 769/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk769<F: Float>(t11465: F, t315: F, t11132: F, t11337: F, t3010: F, t963: F, t3013: F, t323: F, t1034: F, t360: F, t11244: F, t11240: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11466 = t315 * t11465;
    let t11479 = F::cast_from(0.93932222222222222223e0_f64) * t11132;
    let t11480 = F::cast_from(0.36793333333333333333e0_f64) * t11337;
    let t11506 = F::cast_from(1.0_f64) / t3010 / t963;
    let t11507 = t315 * t11506;
    let t11509 = F::cast_from(1.0_f64) / t3013 / t323;
    let t11534 = F::cast_from(0.55403703703703703703e-1_f64) * t11132;
    let t11560 = F::cast_from(0.28842592592592592592e-1_f64) * t11132;
    let t11574 = F::cast_from(0.53272592592592592592e-1_f64) * t11132;
    let t11626 = t1034 * t1034;
    let t11627 = F::cast_from(1.0_f64) / t11626;
    let t11628 = t11627 * t360;
    let t11629 = t11628 * t11244;
    let t11630 = t11240 * t11629;
    (t11466, t11479, t11480, t11506, t11507, t11509, t11534, t11560, t11574, t11627, t11630)
}
