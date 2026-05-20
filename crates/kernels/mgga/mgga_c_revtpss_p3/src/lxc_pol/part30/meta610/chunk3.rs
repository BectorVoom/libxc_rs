//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2087/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2087<F: Float>(t1032: F, t5710: F, t1426: F, t7063: F, t7286: F, t27852: F, t689: F, t25904: F, t27909: F, t4078: F, t94729: F, t94733: F, t94735: F, t94749: F, t94756: F, t94758: F, t97943: F, t97945: F, t97949: F, t97951: F, t97953: F, t97956: F) -> (F, F, F, F) {
    let t97960 = t5710 * t1032;
    let t97961 = t97960 * t1426;
    let t97962 = t7063 * t97961;
    let t97964 = F::cast_from(0.25702851531048074406e-1_f64) * t97962 * t7286;
    let t97966 = t27852 * t689;
    let t97968 = F::cast_from(0.14456046980341999104e-1_f64) * t25904 * t97966;
    let t97969 = F::cast_from(0.13170898365871023197e1_f64) * t27909 * t4078 - F::cast_from(0.10975748638225852664e-1_f64) * t94729 + t97943 + t97945 - F::cast_from(0.13009920719177044025e-2_f64) * t94733 - t97949 + t97951 - t97953 - F::cast_from(0.2601984143835408805e-1_f64) * t94735 + F::cast_from(0.24093411633903331839e-3_f64) * t97956 - F::cast_from(0.19514881078765566038e-1_f64) * t94749 - F::cast_from(0.19274729307122665471e-1_f64) * t94756 - t97964 + F::cast_from(0.14634331517634470219e-1_f64) * t94758 - t97968;
    (t97960, t97961, t97966, t97969)
}
