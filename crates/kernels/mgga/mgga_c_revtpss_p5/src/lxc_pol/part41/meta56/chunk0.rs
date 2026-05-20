//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 339/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk339<F: Float>(t1043: F, t1089: F, t378: F, t1071: F, t380: F, t1024: F, t1083: F, t1087: F, t342: F, t381: F, t989: F) -> (F, F, F) {
    let t1090 = t378 * t1043 * t1089;
    let t1093 = t380 * t1071;
    let t1096 = F::cast_from(0.65854491829355115987e0_f64) * t989 * t381 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t1083 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t1090 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t1093;
    (t1090, t1093, t1096)
}
