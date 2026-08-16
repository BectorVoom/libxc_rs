//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 689/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk689<F: Float>(t1079: F, t3325: F, t1000: F, t1073: F, t1076: F, t1097: F, t3043: F, t3047: F, t3052: F, t3058: F, t3060: F, t3063: F, t3067: F, t3076: F, t3261: F, t3264: F, t3271: F, t342: F, t386: F, t989: F, t995: F) -> (F, F) {
    let t3326 = t1079 * t3325;
    let t3329 = F::cast_from(0.65854491829355115987e0_f64) * t3043 * t386 - F::cast_from(0.13170898365871023197e1_f64) * t3047 * t1000 + F::cast_from(0.13170898365871023197e1_f64) * t989 * t1073 - F::cast_from(0.13170898365871023197e1_f64) * t3052 * t1097 + F::cast_from(0.13170898365871023197e1_f64) * t3058 * t3060 - F::cast_from(0.13170898365871023197e1_f64) * t3063 * t1000 + F::cast_from(0.13170898365871023197e1_f64) * t995 * t3067 - F::cast_from(0.65854491829355115987e0_f64) * t995 * t3076 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t3261 - F::cast_from(0.13170898365871023197e1_f64) * t3264 * t1097 + F::cast_from(0.13170898365871023197e1_f64) * t1076 * t3271 - F::cast_from(0.65854491829355115987e0_f64) * t1076 * t3326;
    (t3326, t3329)
}
