//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 588/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk588<F: Float>(t136: F, t854: F, t221: F, t775: F, t2674: F, t26: F, t66: F) -> (F, F, F, F) {
    let t2675 = t854 * t136;
    let t2677 = t2675 * t221 * t775;
    let t2678 = t2674 * t2677;
    let t2681 = F::cast_from(1.0_f64) / t66 / t26;
    (t2675, t2677, t2678, t2681)
}
