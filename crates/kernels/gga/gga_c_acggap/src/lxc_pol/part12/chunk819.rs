//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 819/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk819<F: Float>(t7963: F, t9168: F, t8306: F, t8406: F, t7942: F, t8453: F, t8459: F, t8494: F, t8129: F, t8447: F, t8451: F, t8455: F, t8466: F, t8470: F, t8474: F, t8478: F, t8482: F, t8487: F, t8492: F, t8498: F) -> (F, F, F, F) {
    let t9169 = t7963 * t9168;
    let t9171 = t8306 * t8406;
    let t9172 = t7942 * t9171;
    let t9176 = F::new(0.85748036236139473944e-3) * t8453;
    let t9178 = F::new(0.15724046144802076034e-2) * t8459;
    let t9186 = F::new(0.42874018118069736972e-3) * t8494;
    let t9188 = F::new(0.31448092289604152069e-2) * t8447 + F::new(0.18868855373762491241e-2) * t8451 + t9176 - F::new(0.34299214494455789578e-2) * t8455 + t8129 - t9178 - F::new(0.94344276868812456204e-2) * t8466 + F::new(0.31448092289604152068e-2) * t8470 - F::new(0.47172138434406228102e-2) * t8474 + F::new(0.62896184579208304138e-3) * t8478 - F::new(0.21437009059034868486e-3) * t8482 + F::new(0.94344276868812456207e-3) * t8487 + F::new(0.62896184579208304138e-3) * t8492 - t9186 - F::new(0.42874018118069736972e-3) * t8498;
    (t9169, t9171, t9172, t9188)
}
