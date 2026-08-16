//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 930/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk930(t76618: f64, t73691: f64, t73693: f64, t73696: f64, t73708: f64, t73714: f64, t73719: f64, t73724: f64, t73729: f64, t73734: f64, t73739: f64, t76604: f64, t76607: f64, t76608: f64, t76613: f64, t76617: f64) -> f64 {
    let t76619 = 0.99317399751028291929e-5_f64 * t76618;
    let t76627 = -t76604 - t73691 - 0.58171619854173713846e-5_f64 * t73693 + 0.58171619854173713846e-5_f64 * t73696 - t76607 - t76608 - t76613 + t76617 + t76619 - 0.87596530464506835935e-6_f64 * t73708 - 0.35038612185802734376e-6_f64 * t73714 - 0.4379826523225341797e-6_f64 * t73719 - 0.35038612185802734376e-6_f64 * t73724 + 0.52557918278704101564e-6_f64 * t73729 - 0.52557918278704101564e-6_f64 * t73734 - 0.17519306092901367188e-6_f64 * t73739;
    t76627
}
