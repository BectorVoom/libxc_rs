//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1625/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1625(t1079: f64, t16327: f64, t342: f64, t4930: f64, t1071: f64, t1647: f64, t1695: f64, t3059: f64, t1651: f64, t3325: f64, t1076: f64, t1097: f64, t11195: f64, t16312: f64, t16314: f64, t16318: f64, t16322: f64, t1696: f64, t3052: f64, t3058: f64, t3067: f64, t3271: f64, t3326: f64, t4752: f64, t4778: f64, t4935: f64, t5016: f64, t995: f64) -> f64 {
    let t16328 = t1079 * t16327;
    let t16333 = t342 * t4930;
    let t16340 = t1647 * t1071;
    let t16343 = t1695 * t3059;
    let t16344 = t1079 * t16343;
    let t16352 = t1079 * t1651 * t3325;
    let t16355 = -0.26341796731742046394e1_f64 * t16312 * t16314 + 0.13170898365871023197e1_f64 * t1076 * t16318 - 0.39512695097613069591e1_f64 * t1076 * t16322 - 0.65854491829355115987e0_f64 * t4752 * t3326 + 0.13170898365871023197e1_f64 * t995 * t16328 + 0.13170898365871023197e1_f64 * t4935 * t3271 - 0.13170898365871023197e1_f64 * t16333 * t1097 + 0.13170898365871023197e1_f64 * t4778 * t3067 - 0.65854491829355115987e0_f64 * t11195 * t1696 - 0.13170898365871023197e1_f64 * t16340 * t1097 - 0.13170898365871023197e1_f64 * t3058 * t16344 - 0.13170898365871023197e1_f64 * t3052 * t5016 - 0.65854491829355115987e0_f64 * t4935 * t3326 + 0.65854491829355115987e0_f64 * t995 * t16352;
    t16355
}
