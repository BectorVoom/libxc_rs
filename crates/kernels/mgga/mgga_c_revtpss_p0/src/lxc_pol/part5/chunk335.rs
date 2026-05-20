//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 335/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk335<F: Float>(t1079: F, t1096: F, t1000: F, t1073: F, t1076: F, t342: F, t386: F, t989: F, t995: F, t389: F) -> (F, F, F) {
    let t1097 = t1079 * t1096;
    let t1100 = F::cast_from(0.65854491829355115987e0_f64) * t989 * t386 - F::cast_from(0.65854491829355115987e0_f64) * t995 * t1000 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t1073 - F::cast_from(0.65854491829355115987e0_f64) * t1076 * t1097;
    let t1102 = F::new(1.0) / t389;
    (t1097, t1100, t1102)
}
