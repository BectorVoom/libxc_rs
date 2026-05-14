//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 525/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk525<F: Float>(t1079: F, t3325: F, t1000: F, t1073: F, t1076: F, t1097: F, t3043: F, t3047: F, t3052: F, t3058: F, t3060: F, t3063: F, t3067: F, t3076: F, t3261: F, t3264: F, t3271: F, t342: F, t386: F, t989: F, t995: F) -> (F, F) {
    let t3326 = t1079 * t3325;
    let t3329 = 0.65854491829355115987e0 * t3043 * t386 - 0.13170898365871023197e1 * t3047 * t1000 + 0.13170898365871023197e1 * t989 * t1073 - 0.13170898365871023197e1 * t3052 * t1097 + 0.13170898365871023197e1 * t3058 * t3060 - 0.13170898365871023197e1 * t3063 * t1000 + 0.13170898365871023197e1 * t995 * t3067 - 0.65854491829355115987e0 * t995 * t3076 + 0.65854491829355115987e0 * t342 * t3261 - 0.13170898365871023197e1 * t3264 * t1097 + 0.13170898365871023197e1 * t1076 * t3271 - 0.65854491829355115987e0 * t1076 * t3326;
    (t3326, t3329)
}
