//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 531/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk531<F: Float>(t112: F, t2289: F, t625: F, t666: F, t111: F, t654: F) -> (F, F, F) {
    let t2335 = F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t2289 * t112;
    let t2336 = t625 * t666;
    let t2339 = F::cast_from(1.0_f64) / t654 / t111;
    (t2335, t2336, t2339)
}
