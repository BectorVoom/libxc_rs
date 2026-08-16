//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 579/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk579<F: Float>(t2777: F, t870: F, t2439: F, t123: F, t212: F, t676: F) -> (F, F, F) {
    let t2778 = t2777 * t870;
    let t2780 = F::cast_from(0.65049603595885220126e-3_f64) * t2439 * t2778;
    let t2782 = t123 * t676 * t212;
    (t2778, t2780, t2782)
}
