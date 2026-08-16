//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1048/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1048(t39122: f64, t34927: f64, t34931: f64, t37266: f64, t39084: f64, t39089: f64, t39094: f64, t39099: f64, t39104: f64, t39108: f64, t39112: f64, t39119: f64, t39127: f64, t39132: f64, t39137: f64, t39142: f64, t5218: f64, t699: f64, t903: f64) -> f64 {
    let t42856 = 0.66211599834018861287e-4_f64 * t39122;
    let t42861 = 0.85129199786595678799e-5_f64 * t39084 - 0.2553875993597870364e-4_f64 * t39089 + 0.1915406995198402773e-3_f64 * t39094 + 0.638468998399467591e-4_f64 * t39099 - 0.638468998399467591e-4_f64 * t39104 - 0.5107751987195740728e-4_f64 * t39108 - 0.1702583995731913576e-4_f64 * t39112 + 0.17961362552795712846e0_f64 * t903 * t699 * t5218 - t37266 + 0.13680077012009379e-5_f64 * t34927 + 0.13680077012009379e-5_f64 * t34931 + 0.20496175532535769482e-3_f64 * t39119 + t42856 + 0.5107751987195740728e-4_f64 * t39127 - 0.212822999466489197e-4_f64 * t39132 + 0.638468998399467591e-4_f64 * t39137 + 0.638468998399467591e-4_f64 * t39142;
    t42861
}
