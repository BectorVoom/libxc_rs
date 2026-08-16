//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 819/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk819(t7963: f64, t9168: f64, t8306: f64, t8406: f64, t7942: f64, t8453: f64, t8459: f64, t8494: f64, t8129: f64, t8447: f64, t8451: f64, t8455: f64, t8466: f64, t8470: f64, t8474: f64, t8478: f64, t8482: f64, t8487: f64, t8492: f64, t8498: f64) -> (f64, f64, f64, f64) {
    let t9169 = t7963 * t9168;
    let t9171 = t8306 * t8406;
    let t9172 = t7942 * t9171;
    let t9176 = 0.85748036236139473944e-3_f64 * t8453;
    let t9178 = 0.15724046144802076034e-2_f64 * t8459;
    let t9186 = 0.42874018118069736972e-3_f64 * t8494;
    let t9188 = 0.31448092289604152069e-2_f64 * t8447 + 0.18868855373762491241e-2_f64 * t8451 + t9176 - 0.34299214494455789578e-2_f64 * t8455 + t8129 - t9178 - 0.94344276868812456204e-2_f64 * t8466 + 0.31448092289604152068e-2_f64 * t8470 - 0.47172138434406228102e-2_f64 * t8474 + 0.62896184579208304138e-3_f64 * t8478 - 0.21437009059034868486e-3_f64 * t8482 + 0.94344276868812456207e-3_f64 * t8487 + 0.62896184579208304138e-3_f64 * t8492 - t9186 - 0.42874018118069736972e-3_f64 * t8498;
    (t9169, t9171, t9172, t9188)
}
