//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 777/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk777<F: Float>(t28455: F, t600: F, t1674: F, t28338: F, t28343: F, t28346: F, t28352: F, t28354: F, t28356: F, t28360: F, t28441: F, t28444: F, t45: F, t6851: F, t8592: F, t16541: F, t8550: F) -> (F, F, F) {
    let t28456 = t28455 * t600;
    let t28459 = 0.35089340384731224426e1 * t1674 * t28338 - 0.35089340384731224426e1 * t1674 * t28343 - 0.51947267698127589897e2 * t1674 * t28346 + 0.35089340384731224426e1 * t6851 * t8592 + t28352 + t28354 + t28356 - t28360 + t28441 + t28444 + 0.19751789702565206229e-1 * t45 * t28456;
    let t28461 = 6.0 * t16541 * t8550;
    (t28456, t28459, t28461)
}
