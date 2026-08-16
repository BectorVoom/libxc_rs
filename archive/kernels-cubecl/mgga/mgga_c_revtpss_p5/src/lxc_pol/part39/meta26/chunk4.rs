//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 173/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk173<F: Float>(t33: F, t516: F, t153: F, t515: F, t162: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t517 = t516 * t33;
    let t518 = piecewise3::<F>(t34, t153, t517);
    let t519 = t515 + t518 - F::cast_from(2.0_f64);
    let t520 = t519 * t162;
    (t517, t520)
}
