//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2384/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2384(t47705: f64, t47707: f64, t47730: f64, t47681: f64, t47686: f64, t47691: f64, t47695: f64, t47699: f64, t47703: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47722: f64, t47724: f64, t47728: f64, t47732: f64, t47736: f64, t47738: f64) -> f64 {
    let t48946 = 8.0_f64 / 9.0_f64 * t47705;
    let t48947 = 8.0_f64 / 27.0_f64 * t47707;
    let t48956 = 4.0_f64 / 9.0_f64 * t47730;
    let t48960 = -80.0_f64 / 81.0_f64 * t47681 + 4.0_f64 * t47686 - 2.0_f64 / 3.0_f64 * t47691 - 2.0_f64 / 3.0_f64 * t47695 - 2.0_f64 / 9.0_f64 * t47699 - 6.0_f64 * t47703 + t48946 - t48947 + 4.0_f64 / 9.0_f64 * t47709 + 2.0_f64 / 9.0_f64 * t47711 + 10.0_f64 / 27.0_f64 * t47713 - 4.0_f64 / 3.0_f64 * t47715 - 2.0_f64 / 3.0_f64 * t47717 - 10.0_f64 / 9.0_f64 * t47722 - 4.0_f64 / 3.0_f64 * t47724 - 8.0_f64 * t47728 - t48956 + t47732 / 3.0_f64 - t47736 / 3.0_f64 + 2.0_f64 * t47738;
    t48960
}
