//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1257/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1257(t93281: f64, t93282: f64, t10509: f64, t25377: f64, t25375: f64, t25296: f64, t25365: f64, t10978: f64, t213: f64, t225: f64, t231: f64, t25286: f64, t25383: f64, t25391: f64, t25392: f64, t25426: f64, t257: f64, t7053: f64, t7070: f64, t7076: f64, t836: f64, t92937: f64, t93099: f64, t93252: f64, t93262: f64, t93267: f64, t93272: f64, t93273: f64, t93276: f64, t93278: f64) -> (f64, f64) {
    let t93283 = t93281 * t93282;
    let t93285 = t25377 * t10509;
    let t93286 = t25375 * t93285;
    let t93297 = t25365 * t25296;
    let t93299 = 0.34697458558045176417e-2_f64 * t93252 + 0.13010442282307799193e1_f64 * t25383 * t25426 + 0.13010442282307799193e1_f64 * t7070 * t7076 * t92937 * t231 + 0.58544643236296698113e-1_f64 * t93262 - 0.65854491829355115987e0_f64 * t7053 * t10978 - 0.26020884564615598386e1_f64 * t25391 * t25392 * t93267 + t93272 + 0.39029762157531132076e-1_f64 * t93273 - t93276 + t93278 + 0.13010442282307799194e0_f64 * t93283 + 0.57824187921367996415e-1_f64 * t93286 + 0.65854491829355115987e0_f64 * t213 * t93099 * t225 * t257 + 0.13010442282307799193e1_f64 * t7070 * t7076 * t25286 * t836 * t231 - 0.77108554593144223218e-1_f64 * t93297;
    (t93285, t93299)
}
