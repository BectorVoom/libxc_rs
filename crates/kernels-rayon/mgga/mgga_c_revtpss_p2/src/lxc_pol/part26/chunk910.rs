//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 910/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk910(t11248: f64, t11632: f64, t1042: f64, t2251: f64, t999: f64, t4801: f64, t1041: f64, t1047: f64, t1063: f64, t11233: f64, t11246: f64, t11252: f64, t11256: f64, t11259: f64, t11264: f64, t11268: f64, t11271: f64, t11274: f64, t11277: f64, t11281: f64, t11286: f64, t11623: f64, t11630: f64, t3124: f64, t3127: f64, t3136: f64, t3157: f64, t3164: f64) -> f64 {
    let t11633 = t11248 * t11632;
    let t11634 = t1042 * t11633;
    let t11637 = t2251 * t999;
    let t11638 = t4801 * t11637;
    let t11639 = t1042 * t11638;
    let t11642 = -0.85748036236139473944e-3_f64 * t1063 * t11233 + 0.64311027177104605458e-3_f64 * t3124 * t3136 - 0.12862205435420921092e-2_f64 * t11246 * t11252 + 0.21437009059034868486e-3_f64 * t11256 * t11259 - 0.14291339372689912324e-3_f64 * t11264 + 0.21722835846488666732e-1_f64 * t11268 * t1047 - 0.45732285992607719436e-2_f64 * t11271 + 0.12862205435420921092e-2_f64 * t11274 * t3157 - 0.64311027177104605458e-3_f64 * t11277 * t3164 - 0.42874018118069736972e-3_f64 * t3127 * t11281 - 0.7145669686344956162e-3_f64 * t3127 * t11286 + 0.21437009059034868486e-3_f64 * t1041 * t11623 + 0.12862205435420921092e-2_f64 * t11630 * t11634 + 0.85748036236139473944e-3_f64 * t3127 * t11639;
    t11642
}
