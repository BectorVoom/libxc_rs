//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1343/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1343<F: Float>(t1149: F, t6474: F, t12248: F, t12297: F, t12397: F, t16706: F, t16708: F, t17010: F, t17011: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F) -> (F, F) {
    let t20580 = t6474 * t1149;
    let t20582 = F::cast_from(0.96491876992155210402e2_f64) * t12248 * t20580;
    let t20597 = -t12397 + F::cast_from(0.76103703703703703703e-2_f64) * t12297 + F::cast_from(0.1522074074074074074e-1_f64) * t16706 + F::cast_from(0.761037037037037037e-2_f64) * t16708 - t17010 - t17011 + F::cast_from(0.3805185185185185185e-2_f64) * t20283 + F::cast_from(0.19025925925925925925e-1_f64) * t20295 - F::cast_from(0.68493333333333333331e-1_f64) * t20300 - F::cast_from(0.2283111111111111111e-1_f64) * t20304 - F::cast_from(0.11415555555555555555e-1_f64) * t20285 + F::cast_from(0.10274e0_f64) * t20308 + F::cast_from(0.68493333333333333332e-1_f64) * t20312 - F::cast_from(0.57077777777777777777e-2_f64) * t20287 - F::cast_from(0.11415555555555555555e-1_f64) * t20315 + F::cast_from(0.34246666666666666666e-1_f64) * t20320 + F::cast_from(0.17123333333333333333e-1_f64) * t20290;
    (t20582, t20597)
}
