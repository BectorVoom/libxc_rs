//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3271/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3271<F: Float>(t14686: F, t18525: F, t50570: F, t61956: F, t14923: F, t18428: F, t10760: F, t40627: F, t61837: F, t18527: F, t50295: F, t18444: F, t2745: F, t2754: F, t40801: F, t40804: F, t40810: F, t4364: F, t51000: F, t51006: F, t51026: F, t51028: F) -> F {
    let t62105 = t50570 * t14686 * t61956 * t18525;
    let t62108 = t14923 * t18428;
    let t62111 = t10760 * t40627 * t61837;
    let t62114 = t50295 * t18527;
    let t62123 = -F::cast_from(0.12004725073059526352e-1_f64) * t51000 - F::cast_from(0.15246000842785598468e-3_f64) * t62105 - F::cast_from(0.80031500487063509015e-2_f64) * t51006 + F::cast_from(0.16006300097412701803e-1_f64) * t62108 + F::cast_from(0.36143185997963725434e-4_f64) * t62111 + F::cast_from(0.90357964994909313582e-5_f64) * t40801 + F::cast_from(0.12004725073059526352e-1_f64) * t62114 - F::cast_from(0.50820002809285328225e-4_f64) * t40804 - F::cast_from(0.21437009059034868486e-3_f64) * t2745 * t4364 * t18444 * t2754 + t40810 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t51026 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t51028;
    t62123
}
