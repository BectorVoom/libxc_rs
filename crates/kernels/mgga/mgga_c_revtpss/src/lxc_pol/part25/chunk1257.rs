//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1257/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1257<F: Float>(t93281: F, t93282: F, t10509: F, t25377: F, t25375: F, t25296: F, t25365: F, t10978: F, t213: F, t225: F, t231: F, t25286: F, t25383: F, t25391: F, t25392: F, t25426: F, t257: F, t7053: F, t7070: F, t7076: F, t836: F, t92937: F, t93099: F, t93252: F, t93262: F, t93267: F, t93272: F, t93273: F, t93276: F, t93278: F) -> (F, F) {
    let t93283 = t93281 * t93282;
    let t93285 = t25377 * t10509;
    let t93286 = t25375 * t93285;
    let t93297 = t25365 * t25296;
    let t93299 = F::new(0.34697458558045176417e-2) * t93252 + F::new(0.13010442282307799193e1) * t25383 * t25426 + F::new(0.13010442282307799193e1) * t7070 * t7076 * t92937 * t231 + F::new(0.58544643236296698113e-1) * t93262 - F::new(0.65854491829355115987e0) * t7053 * t10978 - F::new(0.26020884564615598386e1) * t25391 * t25392 * t93267 + t93272 + F::new(0.39029762157531132076e-1) * t93273 - t93276 + t93278 + F::new(0.13010442282307799194e0) * t93283 + F::new(0.57824187921367996415e-1) * t93286 + F::new(0.65854491829355115987e0) * t213 * t93099 * t225 * t257 + F::new(0.13010442282307799193e1) * t7070 * t7076 * t25286 * t836 * t231 - F::new(0.77108554593144223218e-1) * t93297;
    (t93285, t93299)
}
