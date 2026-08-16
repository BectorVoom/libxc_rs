//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1309/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1309(t1014: f64, t27851: f64, t13273: f64, t303: f64, t7726: f64, t13132: f64, t13150: f64, t13173: f64, t14492: f64, t14497: f64, t14554: f64, t26679: f64, t26685: f64, t26695: f64, t26748: f64, t27812: f64, t27822: f64, t27954: f64, t4947: f64, t7703: f64, t93366: f64, t93485: f64, t95621: f64, t95713: f64, t95721: f64) -> (f64, f64, f64) {
    let t96068 = t1014 * t27851;
    let t96071 = t303 * t7726 * t13273;
    let t96099 = -0.58958024691358024689e-2_f64 * t96068 + 0.13265555555555555555e-1_f64 * t96071 + 0.46336805555555555556e-3_f64 * t7703 * t4947 * t26679 * t13150 + 0.18534722222222222222e-2_f64 * t7703 * t14554 * t26679 * t13173 + 0.6183646701388888889e-4_f64 * t93366 * t27822 + 0.30918233506944444445e-4_f64 * t26685 * t95713 - 0.61890573922526041668e-5_f64 * t27812 * t95621 + 0.12367293402777777778e-3_f64 * t26685 * t95721 - 0.72079475308641975309e-3_f64 * t7703 * t14492 * t93485 * t13132 - 0.12356481481481481482e-2_f64 * t7703 * t14497 * t26695 * t13173 + 0.46336805555555555556e-3_f64 * t26748 * t27954;
    (t96068, t96071, t96099)
}
