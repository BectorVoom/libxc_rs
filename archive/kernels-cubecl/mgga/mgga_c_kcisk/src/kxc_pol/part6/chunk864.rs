//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 864/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk864<F: Float>(t28455: F, t600: F, t1674: F, t28338: F, t28343: F, t28346: F, t28352: F, t28354: F, t28356: F, t28360: F, t28441: F, t28444: F, t45: F, t6851: F, t8592: F) -> (F, F) {
    let t28456 = t28455 * t600;
    let t28459 = F::cast_from(0.35089340384731224426e1_f64) * t1674 * t28338 - F::cast_from(0.35089340384731224426e1_f64) * t1674 * t28343 - F::cast_from(0.51947267698127589897e2_f64) * t1674 * t28346 + F::cast_from(0.35089340384731224426e1_f64) * t6851 * t8592 + t28352 + t28354 + t28356 - t28360 + t28441 + t28444 + F::cast_from(0.19751789702565206229e-1_f64) * t45 * t28456;
    (t28456, t28459)
}
