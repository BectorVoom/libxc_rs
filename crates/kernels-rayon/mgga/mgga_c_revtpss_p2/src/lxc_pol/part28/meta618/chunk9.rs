//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2174/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2174(t25387: f64, t99125: f64, t2723: f64, t836: f64, t886: f64, t14978: f64, t15038: f64, t1558: f64, t1949: f64, t1956: f64, t1957: f64, t231: f64, t233: f64, t25317: f64, t25349: f64, t25391: f64, t25419: f64, t27199: f64, t27275: f64, t27357: f64, t2828: f64, t7053: f64, t7070: f64, t7071: f64, t7076: f64, t7083: f64, t7769: f64, t93112: f64, t93116: f64, t93124: f64, t98922: f64, t99119: f64, t99127: f64) -> f64 {
    let t99147 = 0.51405703062096148812e-1_f64 * t25387 * t99125;
    let t99155 = t2723 * t886 * t836;
    let t99159 = -0.4336814094102599731e0_f64 * t1956 * t1957 * t233 * t99119 - t99127 + 0.13170898365871023197e1_f64 * t7053 * t15038 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t1949 * t14978 - 0.8673628188205199462e0_f64 * t27199 * t25419 + 0.4336814094102599731e0_f64 * t27199 * t25349 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t98922 * t231 - 0.8673628188205199462e0_f64 * t27275 * t7083 - 0.48186823267806663678e-3_f64 * t93112 - 0.48186823267806663678e-3_f64 * t93116 + t99147 + 0.12851425765524037203e-1_f64 * t93124 - 0.26020884564615598386e1_f64 * t7070 * t25317 * t7769 * t2828 + 0.34694512752820797848e1_f64 * t25391 * t27357 * t1558 * t99155;
    t99159
}
