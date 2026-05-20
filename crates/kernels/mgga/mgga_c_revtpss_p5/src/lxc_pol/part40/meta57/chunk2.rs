//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 349/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk349<F: Float>(t1000: F, t1073: F, t1076: F, t1097: F, t342: F, t386: F, t989: F, t995: F, t389: F) -> (F, F) {
    let t1100 = F::cast_from(0.65854491829355115987e0_f64) * t989 * t386 - F::cast_from(0.65854491829355115987e0_f64) * t995 * t1000 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t1073 - F::cast_from(0.65854491829355115987e0_f64) * t1076 * t1097;
    let t1102 = F::new(1.0) / t389;
    (t1100, t1102)
}
