//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2180/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2180(t25411: f64, t99495: f64, t14495: f64, t14979: f64, t25391: f64, t25392: f64, t27189: f64, t27265: f64, t27353: f64, t27357: f64, t27358: f64, t2772: f64, t51608: f64, t7053: f64, t7070: f64, t7071: f64, t7073: f64, t886: f64, t92864: f64, t93387: f64, t93389: f64, t93391: f64, t99237: f64, t99303: f64, t99496: f64, t99502: f64, t99512: f64, t99520: f64) -> f64 {
    let t99522 = t25411 * t99495;
    let t99532 = 0.96373646535613327357e-2_f64 * t99496 - 0.17347256376410398924e1_f64 * t99237 * t27358 - t99502 + 0.17347256376410398924e1_f64 * t99303 * t7073 + 0.13170898365871023197e1_f64 * t27189 * t2772 - 0.65854491829355115987e0_f64 * t7053 * t14979 + 0.8673628188205199462e0_f64 * t27353 * t92864 * t14495 - 0.17347256376410398924e1_f64 * t25391 * t25392 * t99512 + 0.25702851531048074406e-1_f64 * t93387 - 0.14456046980341999104e-1_f64 * t93389 - 0.65049603595885220126e-3_f64 * t99520 - 0.17135234354032049604e-1_f64 * t99522 - 0.8673628188205199462e0_f64 * t27353 * t27357 * t51608 + 0.17347256376410398924e1_f64 * t7070 * t7071 * t27265 * t886 + 0.14634331517634470219e-1_f64 * t93391;
    t99532
}
