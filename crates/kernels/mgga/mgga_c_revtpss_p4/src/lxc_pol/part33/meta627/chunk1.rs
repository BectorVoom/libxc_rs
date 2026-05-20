//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2071/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2071<F: Float>(t99085: F, t2689: F, t27239: F, t25277: F, t4458: F, t14685: F, t14756: F, t7021: F, t14760: F, t93015: F, t27316: F, t686: F, t72: F) -> (F, F, F, F, F, F) {
    let t99086 = F::cast_from(0.10164000561857065645e-3_f64) * t99085;
    let t99091 = t2689 * t27239;
    let t99099 = t25277 * t4458;
    let t99100 = F::new(7.0) / F::new(72.0) * t99099;
    let t99102 = t7021 * t14685 * t14756;
    let t99103 = F::new(7.0) / F::new(24.0) * t99102;
    let t99113 = t93015 * t14760;
    let t99125 = t27316 * t72 * t686;
    (t99086, t99091, t99100, t99103, t99113, t99125)
}
