//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1799/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1799(t25924: f64, t30278: f64, t1903: f64, t8085: f64, t7296: f64, t1904: f64, t213: f64, t25930: f64, t26238: f64, t26251: f64, t26263: f64, t26279: f64, t26294: f64, t27837: f64, t28781: f64, t28783: f64, t28796: f64, t28899: f64, t30227: f64, t30248: f64, t30252: f64, t30257: f64, t30262: f64, t30267: f64, t561: f64, t6896: f64, t7295: f64, t7511: f64, t8100: f64) -> (f64, f64, f64, f64) {
    let t30279 = t25924 * t30278;
    let t30282 = t8085 * t1903;
    let t30283 = t7296 * t30282;
    let t30286 = -0.8673628188205199462e0_f64 * t7295 * t30227 + 0.65854491829355115987e0_f64 * t213 * t30248 * t561 - 0.17347256376410398924e1_f64 * t25930 * t30252 + 0.8673628188205199462e0_f64 * t7295 * t30257 + 0.4336814094102599731e0_f64 * t7295 * t30262 + 0.51405703062096148812e-1_f64 * t28781 + 0.4336814094102599731e0_f64 * t7295 * t30267 + 0.8673628188205199462e0_f64 * t27837 * t8100 + 0.13170898365871023197e1_f64 * t7511 * t6896 - 0.28912093960683998208e-1_f64 * t28783 - t26238 - 0.13170898365871023197e1_f64 * t28899 * t1904 + t26251 - t26263 - 0.25702851531048074406e-1_f64 * t28796 - 0.26020884564615598386e1_f64 * t7295 * t30279 + t26279 - t26294 + 0.17347256376410398924e1_f64 * t7295 * t30283;
    (t30279, t30282, t30283, t30286)
}
