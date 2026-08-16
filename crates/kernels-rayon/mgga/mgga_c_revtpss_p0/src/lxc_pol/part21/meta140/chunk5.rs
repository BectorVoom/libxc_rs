//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 904/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk904(t1000: f64, t1073: f64, t1076: f64, t1097: f64, t3043: f64, t3047: f64, t3052: f64, t3058: f64, t3060: f64, t3063: f64, t3067: f64, t3076: f64, t3261: f64, t3264: f64, t3271: f64, t3326: f64, t342: f64, t386: f64, t989: f64, t995: f64) -> f64 {
    let t3329 = 0.65854491829355115987e0_f64 * t3043 * t386 - 0.13170898365871023197e1_f64 * t3047 * t1000 + 0.13170898365871023197e1_f64 * t989 * t1073 - 0.13170898365871023197e1_f64 * t3052 * t1097 + 0.13170898365871023197e1_f64 * t3058 * t3060 - 0.13170898365871023197e1_f64 * t3063 * t1000 + 0.13170898365871023197e1_f64 * t995 * t3067 - 0.65854491829355115987e0_f64 * t995 * t3076 + 0.65854491829355115987e0_f64 * t342 * t3261 - 0.13170898365871023197e1_f64 * t3264 * t1097 + 0.13170898365871023197e1_f64 * t1076 * t3271 - 0.65854491829355115987e0_f64 * t1076 * t3326;
    t3329
}
