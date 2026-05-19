//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1048/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1048<F: Float>(t39122: F, t34927: F, t34931: F, t37266: F, t39084: F, t39089: F, t39094: F, t39099: F, t39104: F, t39108: F, t39112: F, t39119: F, t39127: F, t39132: F, t39137: F, t39142: F, t5218: F, t699: F, t903: F) -> F {
    let t42856 = F::cast_from(0.66211599834018861287e-4_f64) * t39122;
    let t42861 = F::cast_from(0.85129199786595678799e-5_f64) * t39084 - F::cast_from(0.2553875993597870364e-4_f64) * t39089 + F::cast_from(0.1915406995198402773e-3_f64) * t39094 + F::cast_from(0.638468998399467591e-4_f64) * t39099 - F::cast_from(0.638468998399467591e-4_f64) * t39104 - F::cast_from(0.5107751987195740728e-4_f64) * t39108 - F::cast_from(0.1702583995731913576e-4_f64) * t39112 + F::cast_from(0.17961362552795712846e0_f64) * t903 * t699 * t5218 - t37266 + F::cast_from(0.13680077012009379e-5_f64) * t34927 + F::cast_from(0.13680077012009379e-5_f64) * t34931 + F::cast_from(0.20496175532535769482e-3_f64) * t39119 + t42856 + F::cast_from(0.5107751987195740728e-4_f64) * t39127 - F::cast_from(0.212822999466489197e-4_f64) * t39132 + F::cast_from(0.638468998399467591e-4_f64) * t39137 + F::cast_from(0.638468998399467591e-4_f64) * t39142;
    t42861
}
