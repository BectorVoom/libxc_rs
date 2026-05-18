//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 23/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk23<F: Float>(t6: F, t97: F, t95: F, t64: F, t80: F, t87: F, t90: F, t72: F, t75: F) -> (F, F, F, F) {
    let t98 = t6 * t97;
    let t99 = t95 * t98;
    let t101 = -F::new(0.59778596625315888114e-2) * t64 + F::new(0.1317375e-2) * t80 - F::new(0.23775e-3) * t87 + F::new(0.64744236347453835951e-5) * t90 - F::new(0.540140625e-6) * t99;
    let t103 = F::new(0.11713266981940447749e-2) * t64 * t72 - t75 * t101;
    (t98, t99, t101, t103)
}
