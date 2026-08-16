//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1208/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1208(t30212: f64, t32377: f64, t32379: f64, t32380: f64, t32384: f64, t32385: f64, t32386: f64, t32387: f64, t33936: f64, t36870: f64, t38859: f64, t38863: f64, t38867: f64, t38871: f64, t38875: f64, t38879: f64, t38886: f64) -> f64 {
    let t41382 = 0.18868855373762491241e-2_f64 * t38859 + 0.94344276868812456204e-2_f64 * t38863 - 0.62896184579208304136e-2_f64 * t38867 - 0.37737710747524982482e-2_f64 * t38871 + 0.20965394859736101379e-3_f64 * t38875 - 0.47172138434406228104e-2_f64 * t38879 - t32377 - t32379 + t32380 + t32384 - t32385 - t32386 - t32387 - 0.12579236915841660828e-2_f64 * t30212 - t33936 + 0.10718504529517434243e-2_f64 * t38886 - t36870;
    t41382
}
