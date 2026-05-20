//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1155/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1155<F: Float>(t26842: F, t3598: F, t3594: F, t1238: F, t26817: F, t26821: F, t26822: F, t26824: F, t26827: F, t26832: F, t26836: F, t3606: F, t3663: F, t3674: F, t3689: F, t3694: F, t3701: F, t484: F, t7607: F, t7613: F) -> (F, F, F) {
    let t26843 = t26842 * t3598;
    let t26844 = t3594 * t26843;
    let t26847 = F::cast_from(0.42874018118069736972e-3_f64) * t26817 * t484 - t26821 + F::cast_from(0.57165357490759649296e-3_f64) * t26822 + F::cast_from(0.85748036236139473944e-3_f64) * t26824 * t3674 - F::cast_from(0.85748036236139473944e-3_f64) * t26827 * t1238 - F::cast_from(0.42874018118069736972e-3_f64) * t7613 * t3663 - F::cast_from(0.57165357490759649296e-3_f64) * t26832 + t7607 * t3701 / F::new(216.0) - t26836 / F::new(432.0) - t7607 * t3689 / F::new(288.0) - t7607 * t3694 / F::new(144.0) + F::cast_from(0.85748036236139473944e-3_f64) * t26844 * t3606;
    (t26843, t26844, t26847)
}
