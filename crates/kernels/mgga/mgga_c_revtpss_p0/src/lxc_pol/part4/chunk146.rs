//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 146/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk146<F: Float>(t406: F, t409: F, t412: F, t416: F) -> (F, F, F) {
    let t431 = F::cast_from(0.705945e1_f64) * t409 + F::cast_from(0.1549425e1_f64) * t406 + F::cast_from(0.420775e0_f64) * t412 + F::cast_from(0.1562925e0_f64) * t416;
    let t434 = F::cast_from(1.0_f64) + F::cast_from(0.32163958997385070134e2_f64) / t431;
    let t435 = F::ln(t434);
    (t431, t434, t435)
}
