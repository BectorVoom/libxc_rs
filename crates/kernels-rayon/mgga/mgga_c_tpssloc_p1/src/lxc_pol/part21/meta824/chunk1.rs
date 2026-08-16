//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2896/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2896(t2885: f64, t5737: f64, t2904: f64, t5769: f64, t2842: f64, t2844: f64, t60395: f64, t17423: f64, t2787: f64, t41831: f64, t47705: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47724: f64, t47730: f64, t47732: f64, t48087: f64, t48096: f64, t48098: f64) -> (f64, f64, f64, f64, f64) {
    let t60407 = t5737 * t2885;
    let t60424 = t5769 * t2904;
    let t60429 = 0.32163958997385070134e2_f64 * t2842 * t60395 * t2844;
    let t60434 = 2.0_f64 * t2787 * t17423;
    let t60449 = 0.65725333333333333332e0_f64 * t48087 + 0.10629925925925925926e1_f64 * t47705 - 0.35433086419753086419e0_f64 * t47707 + 0.26574814814814814814e0_f64 * t47709 + 0.13287407407407407407e0_f64 * t47711 + 0.22145679012345679012e0_f64 * t47713 - 0.79724444444444444444e0_f64 * t47715 - 0.39862222222222222222e0_f64 * t47717 - 0.79724444444444444443e0_f64 * t47724 + 0.18257037037037037037e0_f64 * t41831 - 0.36514074074074074074e0_f64 * t48096 + 0.10954222222222222222e0_f64 * t48098 - 0.5314962962962962963e0_f64 * t47730 + 0.19931111111111111111e0_f64 * t47732;
    (t60407, t60424, t60429, t60434, t60449)
}
