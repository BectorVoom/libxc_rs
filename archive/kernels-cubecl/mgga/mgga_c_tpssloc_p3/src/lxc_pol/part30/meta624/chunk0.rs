//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2024/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2024<F: Float>(t23164: F, t23204: F, t25341: F, t1887: F, t81956: F, t25041: F, t215: F, t6581: F, t252: F, t81613: F, t23056: F, t25242: F, t6579: F) -> (F, F, F, F, F, F, F) {
    let t87028 = t23164 * t23204 * t25341;
    let t87029 = F::cast_from(0.16449340668482264365e-1_f64) * t87028;
    let t87049 = t81956 * t1887;
    let t87050 = t87049 * t25041;
    let t87052 = t6581 * t215;
    let t87053 = t81613 * t252;
    let t87057 = t23056 * t215;
    let t87066 = t6579 * t25242;
    (t87029, t87049, t87050, t87052, t87053, t87057, t87066)
}
