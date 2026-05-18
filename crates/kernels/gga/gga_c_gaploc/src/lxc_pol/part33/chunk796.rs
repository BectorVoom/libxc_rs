//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 796/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk796<F: Float>(t7438: F, t7573: F, t2023: F, t2028: F, t2049: F, t2159: F, t2194: F, t2197: F, t2598: F, t2621: F, t2689: F, t2692: F, t5598: F, t5662: F, t7527: F, t7531: F, t7534: F, t7539: F, t7542: F, t7545: F, t7550: F, t7553: F, t7558: F, t7563: F, t7565: F, t7567: F, t7570: F, t7572: F, t797: F, t813: F, t833: F, t955: F) -> F {
    let t7574 = t7573 * t7438;
    let t7577 = -F::new(0.51123901271894332905e0) * t5662 * t2621 + F::new(0.79445533226334281486e-1) * t7527 * t2023 - F::new(0.79445533226334281486e-1) * t7531 * t2028 - F::new(0.79445533226334281486e-1) * t5598 * t7534 - F::new(0.79445533226334281487e-1) * t955 * t2159 - F::new(0.1022478025437886658e1) * t813 * t7539 + F::new(0.79445533226334281487e-1) * t797 * t7542 + F::new(0.1022478025437886658e1) * t833 * t7545 - F::new(0.61348681526273199482e1) * t2194 * t2692 - F::new(0.61348681526273199482e1) * t813 * t7550 + F::new(0.61348681526273199482e1) * t833 * t7553 - F::new(0.47667319935800568892e0) * t2049 * t2689 - F::new(0.47667319935800568892e0) * t797 * t7558 + F::new(0.61348681526273199482e1) * t2197 * t2598 - F::new(0.59584149919750711116e-1) * t7563 + F::new(0.29792074959875355558e-1) * t7565 + F::new(0.14896037479937677779e-1) * t7567 + F::new(0.59584149919750711116e-1) * t7570 + F::new(0.13803453343411469884e2) * t7572 * t7574;
    t7577
}
