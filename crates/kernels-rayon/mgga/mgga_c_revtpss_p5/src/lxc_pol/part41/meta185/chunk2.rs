//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 759/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk759(t1079: f64, t5015: f64, t1000: f64, t1073: f64, t1076: f64, t1097: f64, t1647: f64, t1652: f64, t1680: f64, t1696: f64, t3047: f64, t3052: f64, t3058: f64, t3063: f64, t3264: f64, t342: f64, t386: f64, t4743: f64, t4747: f64, t4752: f64, t4758: f64, t4764: f64, t4773: f64, t4778: f64, t4932: f64, t4935: f64, t4941: f64, t4947: f64, t989: f64, t995: f64) -> (f64, f64) {
    let t5016 = t1079 * t5015;
    let t5019 = 0.65854491829355115987e0_f64 * t4743 * t386 - 0.65854491829355115987e0_f64 * t4747 * t1000 + 0.65854491829355115987e0_f64 * t1647 * t1073 - 0.65854491829355115987e0_f64 * t4752 * t1097 - 0.65854491829355115987e0_f64 * t3047 * t1652 + 0.13170898365871023197e1_f64 * t3058 * t4758 - 0.65854491829355115987e0_f64 * t3063 * t1652 + 0.65854491829355115987e0_f64 * t995 * t4764 - 0.65854491829355115987e0_f64 * t995 * t4773 + 0.65854491829355115987e0_f64 * t989 * t1680 - 0.65854491829355115987e0_f64 * t4778 * t1000 + 0.65854491829355115987e0_f64 * t342 * t4932 - 0.65854491829355115987e0_f64 * t4935 * t1097 - 0.65854491829355115987e0_f64 * t3052 * t1696 + 0.65854491829355115987e0_f64 * t995 * t4941 - 0.65854491829355115987e0_f64 * t3264 * t1696 + 0.13170898365871023197e1_f64 * t1076 * t4947 - 0.65854491829355115987e0_f64 * t1076 * t5016;
    (t5016, t5019)
}
