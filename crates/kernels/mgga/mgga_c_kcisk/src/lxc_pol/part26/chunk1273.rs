//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1273/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1273<F: Float>(t113639: F, t9426: F, t33570: F, t3748: F, t20160: F, t33433: F, t9446: F, t33601: F, t3739: F, t32033: F, t3936: F, t32096: F, t33593: F, t33438: F, t32022: F, t33469: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t113641 = 0.26805555555555555556e-2 * t9426 * t113639;
    let t113642 = t3748 * t33570;
    let t113643 = 0.22109259259259259258e-2 * t113642;
    let t113650 = 0.13888888888888888889e-1 * t9446 * t20160 * t33433;
    let t113666 = t3739 * t33601;
    let t113671 = t3936 * t32033;
    let t113702 = 0.69444444444444444446e-2 * t32096 * t33593;
    let t113708 = t20160 * t33438;
    let t113710 = 0.26805555555555555556e-2 * t9426 * t113708;
    let t113719 = t32022 * t33469;
    (t113641, t113642, t113643, t113650, t113666, t113671, t113702, t113708, t113710, t113719)
}
