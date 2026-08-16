//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2408/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2408<F: Float>(t10650: F, t4396: F, t13655: F, t2787: F, t10810: F, t1561: F, t47705: F, t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t48085: F, t48087: F, t48090: F, t48092: F) -> (F, F, F, F) {
    let t49280 = F::cast_from(3.0_f64) * t10650 * t4396;
    let t49282 = F::cast_from(3.0_f64) * t2787 * t13655;
    let t49285 = t1561 * t10810;
    let t49304 = F::cast_from(0.13772666666666666666e1_f64) * t47705;
    let t49305 = -F::cast_from(0.125034e1_f64) * t48085 + F::cast_from(0.125034e1_f64) * t48087 + F::cast_from(0.62517e0_f64) * t48090 - F::cast_from(0.104195e0_f64) * t48092 - F::cast_from(0.15302962962962962963e1_f64) * t47681 + F::cast_from(0.61977000000000000001e1_f64) * t47686 - F::cast_from(0.103295e1_f64) * t47691 - F::cast_from(0.103295e1_f64) * t47695 - F::cast_from(0.34431666666666666667e0_f64) * t47699 - F::cast_from(0.929655e1_f64) * t47703 + t49304;
    (t49280, t49282, t49285, t49305)
}
