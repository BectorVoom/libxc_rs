//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2232/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2232<F: Float>(t104752: F, t104756: F, t104758: F, t104762: F, t104768: F, t104770: F, t1797: F, t26873: F, t29010: F, t3591: F, t3606: F, t3613: F, t3714: F, t5287: F, t97120: F, t97171: F, t97177: F) -> F {
    let t104772 = -F::cast_from(0.3811023832717309953e-3_f64) * t97171 + F::cast_from(0.42874018118069736972e-3_f64) * t97120 * t1797 + F::cast_from(0.85748036236139473944e-3_f64) * t26873 * t5287 + F::cast_from(0.57165357490759649296e-3_f64) * t104752 * t3714 + t104756 - F::cast_from(0.45732285992607719436e-2_f64) * t104758 * t3606 + F::cast_from(0.22866142996303859718e-2_f64) * t104762 * t3613 + F::cast_from(0.42874018118069736972e-3_f64) * t29010 * t3591 + t104768 - t104770 + t97177 / F::new(648.0);
    t104772
}
