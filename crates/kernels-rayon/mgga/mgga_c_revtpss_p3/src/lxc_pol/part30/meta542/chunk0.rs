//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1971/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1971(t1215: f64, t26922: f64, t26949: f64, t26994: f64, t29264: f64, t29268: f64, t29272: f64, t29275: f64, t29279: f64, t29283: f64, t29287: f64, t29293: f64, t29297: f64, t29301: f64, t29304: f64, t29308: f64, t5237: f64, t5429: f64, t5498: f64, t7602: f64, t7632: f64, t7636: f64, t7639: f64, t7643: f64, t7651: f64) -> f64 {
    let t29311 = 0.65854491829355115987e0_f64 * t7602 * t5237 + 0.13170898365871023197e1_f64 * t7632 * t5429 - 0.26020884564615598386e1_f64 * t26949 * t29264 + 0.17347256376410398924e1_f64 * t26994 * t29268 + 0.8673628188205199462e0_f64 * t7651 * t29272 - 0.8673628188205199462e0_f64 * t29275 * t7639 + 0.8673628188205199462e0_f64 * t7643 * t29279 + 0.8673628188205199462e0_f64 * t7651 * t29283 + 0.17347256376410398924e1_f64 * t26994 * t29287 - 0.65854491829355115987e0_f64 * t7632 * t5498 - 0.17347256376410398924e1_f64 * t7643 * t29293 + 0.17347256376410398924e1_f64 * t7636 * t29297 - 0.8673628188205199462e0_f64 * t7636 * t29301 - 0.65854491829355115987e0_f64 * t29304 * t1215 + 0.8673628188205199462e0_f64 * t26922 * t29308;
    t29311
}
