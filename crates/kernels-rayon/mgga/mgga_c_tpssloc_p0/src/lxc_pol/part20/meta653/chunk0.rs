//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2408/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2408(t10650: f64, t4396: f64, t13655: f64, t2787: f64, t10810: f64, t1561: f64, t47705: f64, t47681: f64, t47686: f64, t47691: f64, t47695: f64, t47699: f64, t47703: f64, t48085: f64, t48087: f64, t48090: f64, t48092: f64) -> (f64, f64, f64, f64) {
    let t49280 = 3.0_f64 * t10650 * t4396;
    let t49282 = 3.0_f64 * t2787 * t13655;
    let t49285 = t1561 * t10810;
    let t49304 = 0.13772666666666666666e1_f64 * t47705;
    let t49305 = -0.125034e1_f64 * t48085 + 0.125034e1_f64 * t48087 + 0.62517e0_f64 * t48090 - 0.104195e0_f64 * t48092 - 0.15302962962962962963e1_f64 * t47681 + 0.61977000000000000001e1_f64 * t47686 - 0.103295e1_f64 * t47691 - 0.103295e1_f64 * t47695 - 0.34431666666666666667e0_f64 * t47699 - 0.929655e1_f64 * t47703 + t49304;
    (t49280, t49282, t49285, t49305)
}
