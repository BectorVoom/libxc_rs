//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2174/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2174<F: Float>(t25387: F, t99125: F, t2723: F, t836: F, t886: F, t14978: F, t15038: F, t1558: F, t1949: F, t1956: F, t1957: F, t231: F, t233: F, t25317: F, t25349: F, t25391: F, t25419: F, t27199: F, t27275: F, t27357: F, t2828: F, t7053: F, t7070: F, t7071: F, t7076: F, t7083: F, t7769: F, t93112: F, t93116: F, t93124: F, t98922: F, t99119: F, t99127: F) -> F {
    let t99147 = F::cast_from(0.51405703062096148812e-1_f64) * t25387 * t99125;
    let t99155 = t2723 * t886 * t836;
    let t99159 = -F::cast_from(0.4336814094102599731e0_f64) * t1956 * t1957 * t233 * t99119 - t99127 + F::cast_from(0.13170898365871023197e1_f64) * t7053 * t15038 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t1949 * t14978 - F::cast_from(0.8673628188205199462e0_f64) * t27199 * t25419 + F::cast_from(0.4336814094102599731e0_f64) * t27199 * t25349 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t98922 * t231 - F::cast_from(0.8673628188205199462e0_f64) * t27275 * t7083 - F::cast_from(0.48186823267806663678e-3_f64) * t93112 - F::cast_from(0.48186823267806663678e-3_f64) * t93116 + t99147 + F::cast_from(0.12851425765524037203e-1_f64) * t93124 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25317 * t7769 * t2828 + F::cast_from(0.34694512752820797848e1_f64) * t25391 * t27357 * t1558 * t99155;
    t99159
}
