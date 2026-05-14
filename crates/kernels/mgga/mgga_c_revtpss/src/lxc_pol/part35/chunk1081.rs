//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1081/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1081<F: Float>(t103449: F, t103463: F, t103471: F, t103490: F, t106275: F, t110600: F, t110613: F, t110615: F, t113373: F, t115573: F, t2067: F, t231: F, t27199: F, t30401: F, t30406: F, t7070: F, t7076: F, t8012: F, t95891: F, t95893: F, t95899: F) -> (F,) {
    let t115658 = 0.15421710918628844643e0 * t110600 - 0.39029762157531132076e-1 * t103449 + t95891 + 0.51405703062096148814e-2 * t103463 - t95893 + 0.13010442282307799193e1 * t106275 * t8012 + 0.13010442282307799193e1 * t7070 * t7076 * t115573 * t231 + 0.26020884564615598386e1 * t27199 * t30401 + 0.13010442282307799193e1 * t27199 * t30406 - 0.4336814094102599731e0 * t113373 * t2067 + t95899 + 0.14456046980341999104e-2 * t103471 - 0.29272321618148349057e-1 * t110613 - 0.43368140941025997312e-1 * t110615 + 0.21951497276451705329e-1 * t103490;
    (t115658,)
}
