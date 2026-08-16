//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1221/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1221(t18687: f64, t18722: f64, t18754: f64, t18782: f64, t868: f64, t10503: f64, t10507: f64, t10511: f64, t10984: f64, t14998: f64, t15004: f64, t15006: f64, t15010: f64, t15015: f64, t18324: f64, t18658: f64, t18663: f64, t213: f64, t257: f64, t865: f64) -> f64 {
    let t18784 = t18687 + t18722 + t18754 + t18782;
    let t18785 = t868 * t18784;
    let t18791 = 0.13170898365871023197e1_f64 * t865 * t18324 - 0.14634331517634470219e-1_f64 * t14998 - t10503 + 0.65854491829355115987e0_f64 * t213 * t18658 * t257 - 0.39512695097613069591e1_f64 * t865 * t18663 - 0.11565819519348392139e-2_f64 * t10507 + 0.13009920719177044025e-1_f64 * t10511 - 0.65854491829355115987e0_f64 * t865 * t18785 - 0.23131639038696784278e-2_f64 * t15004 + t10984 - 0.26019841438354088051e-1_f64 * t15006 + t15010 + 0.13009920719177044025e-2_f64 * t15015;
    t18791
}
