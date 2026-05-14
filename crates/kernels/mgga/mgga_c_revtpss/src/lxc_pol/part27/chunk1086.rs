//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1086/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1086<F: Float>(t7058: F, t92871: F, t1032: F, t11007: F, t233: F, t25372: F, t10996: F, t25377: F, t10509: F, t25375: F, t25296: F, t25365: F, t10978: F, t213: F, t225: F, t231: F, t25286: F, t25383: F, t25391: F, t25392: F, t25426: F, t257: F, t7053: F, t7070: F, t7076: F, t836: F, t92937: F, t93099: F, t93252: F, t93262: F, t93267: F, t93272: F, t93273: F, t93276: F) -> (F, F, F, F) {
    let t93278 = 0.22487184191643109717e-1 * t7058 * t92871;
    let t93279 = t1032 * t11007;
    let t93280 = t93279 * t233;
    let t93281 = t25372 * t93280;
    let t93282 = t25377 * t10996;
    let t93283 = t93281 * t93282;
    let t93285 = t25377 * t10509;
    let t93286 = t25375 * t93285;
    let t93297 = t25365 * t25296;
    let t93299 = 0.34697458558045176417e-2 * t93252 + 0.13010442282307799193e1 * t25383 * t25426 + 0.13010442282307799193e1 * t7070 * t7076 * t92937 * t231 + 0.58544643236296698113e-1 * t93262 - 0.65854491829355115987e0 * t7053 * t10978 - 0.26020884564615598386e1 * t25391 * t25392 * t93267 + t93272 + 0.39029762157531132076e-1 * t93273 - t93276 + t93278 + 0.13010442282307799194e0 * t93283 + 0.57824187921367996415e-1 * t93286 + 0.65854491829355115987e0 * t213 * t93099 * t225 * t257 + 0.13010442282307799193e1 * t7070 * t7076 * t25286 * t836 * t231 - 0.77108554593144223218e-1 * t93297;
    (t93280, t93282, t93285, t93299)
}
