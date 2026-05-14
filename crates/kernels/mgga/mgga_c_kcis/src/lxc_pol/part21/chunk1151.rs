//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1151/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1151<F: Float>(t1014: F, t27851: F, t13273: F, t303: F, t7726: F, t13132: F, t13150: F, t13173: F, t14492: F, t14497: F, t14554: F, t26679: F, t26685: F, t26695: F, t26748: F, t27812: F, t27822: F, t27954: F, t4947: F, t7703: F, t93366: F, t93485: F, t95621: F, t95713: F, t95721: F) -> (F, F, F) {
    let t96068 = t1014 * t27851;
    let t96071 = t303 * t7726 * t13273;
    let t96099 = -0.58958024691358024689e-2 * t96068 + 0.13265555555555555555e-1 * t96071 + 0.46336805555555555556e-3 * t7703 * t4947 * t26679 * t13150 + 0.18534722222222222222e-2 * t7703 * t14554 * t26679 * t13173 + 0.6183646701388888889e-4 * t93366 * t27822 + 0.30918233506944444445e-4 * t26685 * t95713 - 0.61890573922526041668e-5 * t27812 * t95621 + 0.12367293402777777778e-3 * t26685 * t95721 - 0.72079475308641975309e-3 * t7703 * t14492 * t93485 * t13132 - 0.12356481481481481482e-2 * t7703 * t14497 * t26695 * t13173 + 0.46336805555555555556e-3 * t26748 * t27954;
    (t96068, t96071, t96099)
}
