//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2822/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2822(t2: f64, t4324: f64, t584: f64, t1534: f64, t16: f64, t17139: f64, t14389: f64, t48763: f64, t41656: f64, t47705: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47724: f64, t47730: f64, t47732: f64, t47738: f64) -> (f64, f64, f64, f64, f64) {
    let t59627 = 4.0_f64 * t4324 * t2 * t584;
    let t59629 = 2.0_f64 * t1534 * t584;
    let t59631 = 6.0_f64 * t17139 * t16;
    let t59637 = 0.38596750796862084161e3_f64 * t48763 * t14389;
    let t59650 = 0.32962962962962962963e-1_f64 * t47705 - 0.10987654320987654321e-1_f64 * t47707 + 0.82407407407407407408e-2_f64 * t47709 + 0.41203703703703703704e-2_f64 * t47711 + 0.68672839506172839507e-2_f64 * t47713 - 0.24722222222222222222e-1_f64 * t47715 - 0.12361111111111111111e-1_f64 * t47717 - 0.24722222222222222223e-1_f64 * t47724 - 0.16481481481481481482e-1_f64 * t47730 + 0.61805555555555555556e-2_f64 * t47732 + 0.37083333333333333333e-1_f64 * t47738 - 0.41203703703703703703e-2_f64 * t41656;
    (t59627, t59629, t59631, t59637, t59650)
}
