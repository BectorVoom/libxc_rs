//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2176/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2176(t213: f64, t225: f64, t25392: f64, t25395: f64, t257: f64, t27353: f64, t4533: f64, t51525: f64, t51570: f64, t7048: f64, t7070: f64, t7071: f64, t7770: f64, t93126: f64, t93138: f64, t93142: f64, t93143: f64, t93147: f64, t93151: f64, t99119: f64, t99163: f64, t99166: f64, t99174: f64, t99186: f64, t99188: f64, t99191: f64) -> f64 {
    let t99194 = t99163 + 0.8673628188205199462e0_f64 * t93126 * t7770 - 0.73171657588172351096e-2_f64 * t99166 + 0.17347256376410398924e1_f64 * t7070 * t7071 * t7048 * t4533 + t93138 - t93142 + 0.14456046980341999104e-1_f64 * t93143 + 0.72280234901709995518e-2_f64 * t93147 + 0.26020884564615598386e1_f64 * t27353 * t99174 * t51570 - 0.19274729307122665471e-1_f64 * t93151 + 0.65854491829355115987e0_f64 * t213 * t99119 * t225 * t257 + 0.4336814094102599731e0_f64 * t27353 * t25392 * t51525 + 0.13009920719177044025e-1_f64 * t99186 + 0.73171657588172351096e-2_f64 * t99188 - 0.17347256376410398924e1_f64 * t99191 * t25395;
    t99194
}
