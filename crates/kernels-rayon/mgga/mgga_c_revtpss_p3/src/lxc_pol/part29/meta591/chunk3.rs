//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1966/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1966(t28829: f64, t689: f64, t25899: f64, t26271: f64, t27884: f64, t28862: f64, t686: f64, t72: f64, t25895: f64, t102249: f64, t102253: f64, t102255: f64, t102257: f64, t102261: f64, t102266: f64, t25924: f64, t4131: f64, t7295: f64, t8094: f64, t8100: f64, t94610: f64, t96269: f64, t96272: f64, t96277: f64, t96280: f64) -> (f64, f64, f64) {
    let t102268 = t28829 * t689;
    let t102270 = 0.25702851531048074406e-1_f64 * t25899 * t102268;
    let t102272 = 0.25702851531048074406e-1_f64 * t27884 * t26271;
    let t102274 = t28862 * t72 * t686;
    let t102276 = 0.28912093960683998208e-1_f64 * t25895 * t102274;
    let t102282 = -0.73171657588172351096e-2_f64 * t102249 - 0.72280234901709995518e-2_f64 * t96269 + t102253 - t102255 + 0.43368140941025997312e-1_f64 * t96272 + 0.39029762157531132075e-1_f64 * t102257 + t102261 + 0.4336814094102599731e0_f64 * t94610 * t8100 - 0.19274729307122665471e-1_f64 * t96277 + 0.11565819519348392139e-2_f64 * t102266 + t102270 - t102272 - t102276 - 0.68540937416128198418e-2_f64 * t96280 - 0.26020884564615598386e1_f64 * t7295 * t25924 * t8094 * t4131;
    (t102268, t102274, t102282)
}
