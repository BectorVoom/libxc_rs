//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1328/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1328(t10259: f64, t2163: f64, t2331: f64, t2372: f64, t27060: f64, t29432: f64, t651: f64, t671: f64, t94361: f64, t94369: f64, t94371: f64, t94374: f64, t94376: f64, t94940: f64, t94942: f64, t94944: f64, t94998: f64, t95001: f64, t95005: f64, t95008: f64, t95011: f64, t95013: f64, t95015: f64, t96706: f64) -> f64 {
    let t97537 = -2.0_f64 * t10259 * t2163 * t651 - 12.0_f64 * t2331 * t27060 - 6.0_f64 * t2372 * t27060 - 6.0_f64 * t2372 * t29432 - 6.0_f64 * t671 * t96706 + t94361 + t94369 - t94371 - t94374 + t94376 + t94940 - t94942 - t94944 - t94998 + t95001 + t95005 + t95008 - t95011 - t95013 - t95015;
    t97537
}
