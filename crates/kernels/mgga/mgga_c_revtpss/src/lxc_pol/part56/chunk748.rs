//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 748/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk748<F: Float>(t7769: F, t886: F, t25317: F, t225: F, t27265: F, t1579: F, t231: F, t836: F, t25392: F, t7048: F, t7071: F, t7759: F, t25399: F, t4481: F, t1580: F, t213: F, t25322: F, t25362: F, t25364: F, t25366: F, t25368: F, t25371: F, t25379: F, t25391: F, t257: F, t27199: F, t7070: F, t7079: F) -> (F, F, F, F) {
    let t27299 = t7769 * t886;
    let t27300 = t25317 * t27299;
    let t27303 = t27265 * t225;
    let t27312 = t1579 * t836 * t231;
    let t27313 = t25392 * t27312;
    let t27316 = t7048 * t1579;
    let t27317 = t7071 * t27316;
    let t27322 = t7071 * t7759 * t886;
    let t27325 = t25399 * t4481;
    let t27329 = -0.26020884564615598386e1 * t7070 * t27300 - t25362 + 0.65854491829355115987e0 * t213 * t27303 * t257 + 0.4336814094102599731e0 * t27199 * t7079 - t25364 - 0.12851425765524037203e-1 * t25366 - 0.12851425765524037203e-1 * t25368 + t25371 - 0.8673628188205199462e0 * t25391 * t27313 + 0.8673628188205199462e0 * t7070 * t27317 - 0.14456046980341999104e-1 * t25379 + 0.8673628188205199462e0 * t7070 * t27322 - 0.9757440539382783019e-2 * t27325 - 0.65854491829355115987e0 * t25322 * t1580;
    (t27312, t27317, t27322, t27329)
}
