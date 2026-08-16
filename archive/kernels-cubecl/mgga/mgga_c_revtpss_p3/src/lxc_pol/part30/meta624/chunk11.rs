//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2161/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2161<F: Float>(t213: F, t225: F, t25392: F, t25395: F, t257: F, t27353: F, t4533: F, t51525: F, t51570: F, t7048: F, t7070: F, t7071: F, t7770: F, t93126: F, t93138: F, t93142: F, t93143: F, t93147: F, t93151: F, t99119: F, t99163: F, t99166: F, t99174: F, t99186: F, t99188: F, t99191: F) -> F {
    let t99194 = t99163 + F::cast_from(0.8673628188205199462e0_f64) * t93126 * t7770 - F::cast_from(0.73171657588172351096e-2_f64) * t99166 + F::cast_from(0.17347256376410398924e1_f64) * t7070 * t7071 * t7048 * t4533 + t93138 - t93142 + F::cast_from(0.14456046980341999104e-1_f64) * t93143 + F::cast_from(0.72280234901709995518e-2_f64) * t93147 + F::cast_from(0.26020884564615598386e1_f64) * t27353 * t99174 * t51570 - F::cast_from(0.19274729307122665471e-1_f64) * t93151 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t99119 * t225 * t257 + F::cast_from(0.4336814094102599731e0_f64) * t27353 * t25392 * t51525 + F::cast_from(0.13009920719177044025e-1_f64) * t99186 + F::cast_from(0.73171657588172351096e-2_f64) * t99188 - F::cast_from(0.17347256376410398924e1_f64) * t99191 * t25395;
    t99194
}
