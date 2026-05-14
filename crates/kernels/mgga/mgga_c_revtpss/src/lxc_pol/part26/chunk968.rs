//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 968/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk968<F: Float>(t1956: F, t213: F, t25383: F, t257: F, t26437: F, t26439: F, t26441: F, t26448: F, t26475: F, t26483: F, t26486: F, t26489: F, t26493: F, t26498: F, t26500: F, t26502: F, t26508: F, t26511: F, t26515: F, t26521: F, t26522: F, t7067: F, t7070: F, t7415: F, t7424: F) -> (F,) {
    let t26524 = -t26437 + t26439 + 0.8673628188205199462e0 * t7070 * t26441 + 0.17347256376410398924e1 * t25383 * t7415 - 0.10975748638225852664e-1 * t26448 - 0.4336814094102599731e0 * t1956 * t26475 - 0.8673628188205199462e0 * t7067 * t7424 + 0.25702851531048074406e-1 * t26483 + 0.51405703062096148812e-1 * t26486 - 0.26020884564615598386e1 * t7070 * t26489 + 0.17347256376410398924e1 * t7070 * t26493 - 0.19514881078765566038e-1 * t26498 - 0.14456046980341999104e-1 * t26500 + 0.65854491829355115987e0 * t213 * t26502 * t257 + t26508 - 0.8673628188205199462e0 * t7070 * t26511 + 0.4336814094102599731e0 * t7070 * t26515 + t26521 - 0.25702851531048074406e-1 * t26522;
    (t26524,)
}
