//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1190/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1190<F: Float>(t222: F, t7308: F, t7460: F, t2707: F, t7511: F, t2724: F, t7260: F, t1110: F, t2674: F, t2810: F, t7253: F, t1105: F, t16: F, t492: F, t7940: F) -> (F, F, F, F, F) {
    let t22045 = F::new(0.4274e0) * t222 * t7460 * t7308;
    let t22046 = t7511 * t2707;
    let t22050 = F::new(0.14246666666666666666e0) * t222 * t7260 * t2724;
    let t22054 = F::new(0.62337092780453269531e3) * t1110 * t7253 * t2674 * t2810;
    let t22058 = F::new(0.18989649058080861537e-2) * t1105 * t16 * t7940 * t492;
    (t22045, t22046, t22050, t22054, t22058)
}
