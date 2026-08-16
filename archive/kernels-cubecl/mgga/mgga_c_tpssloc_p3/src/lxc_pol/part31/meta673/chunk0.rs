//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2024/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2024<F: Float>(t84533: F, t91305: F, t91312: F, t91314: F, t91323: F, t91346: F, t93720: F, t93722: F, t93731: F, t93736: F, t93742: F, t93743: F, t93745: F, t97378: F, t97380: F, t97382: F, t97387: F, t97389: F) -> F {
    let t102715 = -t93720 + F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t91305 + t93722 - F::cast_from(0.21083550404717759668e-2_f64) * t91312 - t91314 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t97378 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t97380 + t97382 / F::cast_from(384.0_f64) + F::cast_from(0.40372756094140390853e-3_f64) * t91323 + t93731 + F::cast_from(0.24223653656484234512e-2_f64) * t97387 + t97389 / F::cast_from(192.0_f64) - t93736 + F::cast_from(0.6728792682356731809e-4_f64) * t91346 - t84533 - t93742 + t93743 - t93745;
    t102715
}
