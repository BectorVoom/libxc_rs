//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2025/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2025<F: Float>(t84536: F, t91383: F, t91394: F, t93753: F, t97394: F, t97398: F, t97400: F, t97402: F, t97404: F, t97407: F, t97410: F, t97412: F, t97414: F, t97416: F, t97419: F, t97423: F, t97427: F, t97431: F) -> F {
    let t102732 = -t84536 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t97394 - t91383 - F::cast_from(0.40372756094140390853e-3_f64) * t97398 - F::cast_from(0.56521858531796547194e-2_f64) * t97400 - t93753 - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t91394 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t97402 - F::cast_from(0.11869590291677274911e0_f64) * t97404 - F::cast_from(0.33913115119077928317e-1_f64) * t97407 + F::cast_from(0.48447307312968469024e-2_f64) * t97410 - t97412 / F::cast_from(96.0_f64) + t97414 / F::cast_from(192.0_f64) - F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t97416 + t97419 / F::cast_from(8.0_f64) - F::cast_from(0.24223653656484234512e-2_f64) * t97423 - F::cast_from(0.28260929265898273597e-2_f64) * t97427 + F::cast_from(0.40372756094140390853e-3_f64) * t97431;
    t102732
}
