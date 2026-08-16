//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2190/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2190(t2453: f64, t2458: f64, t7760: f64, t25326: f64, t25394: f64, t27199: f64, t887: f64, t93306: f64, t93312: f64, t93315: f64, t93318: f64, t93322: f64, t93324: f64, t93349: f64, t99412: f64, t99414: f64, t99420: f64, t99423: f64, t99425: f64, t99429: f64) -> f64 {
    let t99435 = t2453 * t7760 * t2458;
    let t99440 = 0.19274729307122665472e-1_f64 * t99412 + 0.52041769129231196772e1_f64 * t93349 * t99414 * t25394 + 0.34270468708064099208e-1_f64 * t93306 - t99420 + 0.4818682326780666368e-3_f64 * t99423 - 0.22849835011101738147e-2_f64 * t99425 + 0.8673628188205199462e0_f64 * t27199 * t25326 - 0.13170898365871023197e1_f64 * t99429 * t887 + 0.25702851531048074406e-1_f64 * t93312 + 0.14456046980341999104e-1_f64 * t93315 + 0.11565819519348392139e-2_f64 * t99435 - 0.77108554593144223218e-1_f64 * t93318 - 0.14456046980341999104e-1_f64 * t93322 + 0.34270468708064099208e-1_f64 * t93324;
    t99440
}
