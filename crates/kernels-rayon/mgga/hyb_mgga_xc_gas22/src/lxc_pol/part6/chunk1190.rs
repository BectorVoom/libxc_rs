//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1190/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1190(t222: f64, t7308: f64, t7460: f64, t2707: f64, t7511: f64, t2724: f64, t7260: f64, t1110: f64, t2674: f64, t2810: f64, t7253: f64, t1105: f64, t16: f64, t492: f64, t7940: f64) -> (f64, f64, f64, f64, f64) {
    let t22045 = 0.4274e0_f64 * t222 * t7460 * t7308;
    let t22046 = t7511 * t2707;
    let t22050 = 0.14246666666666666666e0_f64 * t222 * t7260 * t2724;
    let t22054 = 0.62337092780453269531e3_f64 * t1110 * t7253 * t2674 * t2810;
    let t22058 = 0.18989649058080861537e-2_f64 * t1105 * t16 * t7940 * t492;
    (t22045, t22046, t22050, t22054, t22058)
}
