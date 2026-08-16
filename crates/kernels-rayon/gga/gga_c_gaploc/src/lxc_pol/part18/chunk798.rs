//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 798/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk798(t7438: f64, t7573: f64, t2023: f64, t2028: f64, t2049: f64, t2159: f64, t2194: f64, t2197: f64, t2598: f64, t2621: f64, t2689: f64, t2692: f64, t5598: f64, t5662: f64, t7527: f64, t7531: f64, t7534: f64, t7539: f64, t7542: f64, t7545: f64, t7550: f64, t7553: f64, t7558: f64, t7563: f64, t7565: f64, t7567: f64, t7570: f64, t7572: f64, t797: f64, t813: f64, t833: f64, t955: f64) -> f64 {
    let t7574 = t7573 * t7438;
    let t7577 = -0.51123901271894332905e0_f64 * t5662 * t2621 + 0.79445533226334281486e-1_f64 * t7527 * t2023 - 0.79445533226334281486e-1_f64 * t7531 * t2028 - 0.79445533226334281486e-1_f64 * t5598 * t7534 - 0.79445533226334281487e-1_f64 * t955 * t2159 - 0.1022478025437886658e1_f64 * t813 * t7539 + 0.79445533226334281487e-1_f64 * t797 * t7542 + 0.1022478025437886658e1_f64 * t833 * t7545 - 0.61348681526273199482e1_f64 * t2194 * t2692 - 0.61348681526273199482e1_f64 * t813 * t7550 + 0.61348681526273199482e1_f64 * t833 * t7553 - 0.47667319935800568892e0_f64 * t2049 * t2689 - 0.47667319935800568892e0_f64 * t797 * t7558 + 0.61348681526273199482e1_f64 * t2197 * t2598 - 0.59584149919750711116e-1_f64 * t7563 + 0.29792074959875355558e-1_f64 * t7565 + 0.14896037479937677779e-1_f64 * t7567 + 0.59584149919750711116e-1_f64 * t7570 + 0.13803453343411469884e2_f64 * t7572 * t7574;
    t7577
}
