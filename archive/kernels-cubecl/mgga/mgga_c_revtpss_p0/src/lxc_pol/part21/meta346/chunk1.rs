//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1680/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1680<F: Float>(t11637: F, t4801: F, t1042: F, t1041: F, t1047: F, t1063: F, t11233: F, t11246: F, t11252: F, t11256: F, t11259: F, t11264: F, t11268: F, t11271: F, t11274: F, t11277: F, t11281: F, t11286: F, t11623: F, t11630: F, t11634: F, t3124: F, t3127: F, t3136: F, t3157: F, t3164: F) -> (F, F, F) {
    let t11638 = t4801 * t11637;
    let t11639 = t1042 * t11638;
    let t11642 = -F::cast_from(0.85748036236139473944e-3_f64) * t1063 * t11233 + F::cast_from(0.64311027177104605458e-3_f64) * t3124 * t3136 - F::cast_from(0.12862205435420921092e-2_f64) * t11246 * t11252 + F::cast_from(0.21437009059034868486e-3_f64) * t11256 * t11259 - F::cast_from(0.14291339372689912324e-3_f64) * t11264 + F::cast_from(0.21722835846488666732e-1_f64) * t11268 * t1047 - F::cast_from(0.45732285992607719436e-2_f64) * t11271 + F::cast_from(0.12862205435420921092e-2_f64) * t11274 * t3157 - F::cast_from(0.64311027177104605458e-3_f64) * t11277 * t3164 - F::cast_from(0.42874018118069736972e-3_f64) * t3127 * t11281 - F::cast_from(0.7145669686344956162e-3_f64) * t3127 * t11286 + F::cast_from(0.21437009059034868486e-3_f64) * t1041 * t11623 + F::cast_from(0.12862205435420921092e-2_f64) * t11630 * t11634 + F::cast_from(0.85748036236139473944e-3_f64) * t3127 * t11639;
    (t11638, t11639, t11642)
}
