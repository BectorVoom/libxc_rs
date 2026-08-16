//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2099/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2099(t1032: f64, t5710: f64, t1426: f64, t7063: f64, t7286: f64, t27852: f64, t689: f64, t25904: f64, t27909: f64, t4078: f64, t94729: f64, t94733: f64, t94735: f64, t94749: f64, t94756: f64, t94758: f64, t97943: f64, t97945: f64, t97949: f64, t97951: f64, t97953: f64, t97956: f64) -> (f64, f64, f64, f64) {
    let t97960 = t5710 * t1032;
    let t97961 = t97960 * t1426;
    let t97962 = t7063 * t97961;
    let t97964 = 0.25702851531048074406e-1_f64 * t97962 * t7286;
    let t97966 = t27852 * t689;
    let t97968 = 0.14456046980341999104e-1_f64 * t25904 * t97966;
    let t97969 = 0.13170898365871023197e1_f64 * t27909 * t4078 - 0.10975748638225852664e-1_f64 * t94729 + t97943 + t97945 - 0.13009920719177044025e-2_f64 * t94733 - t97949 + t97951 - t97953 - 0.2601984143835408805e-1_f64 * t94735 + 0.24093411633903331839e-3_f64 * t97956 - 0.19514881078765566038e-1_f64 * t94749 - 0.19274729307122665471e-1_f64 * t94756 - t97964 + 0.14634331517634470219e-1_f64 * t94758 - t97968;
    (t97960, t97961, t97966, t97969)
}
