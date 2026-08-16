//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2863/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2863<F: Float>(t41656: F, t47705: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47724: F, t47730: F, t47732: F, t47738: F) -> F {
    let t59846 = F::cast_from(0.63318518518518518517e-1_f64) * t47705 - F::cast_from(0.21106172839506172839e-1_f64) * t47707 + F::cast_from(0.15829629629629629629e-1_f64) * t47709 + F::cast_from(0.79148148148148148147e-2_f64) * t47711 + F::cast_from(0.13191358024691358025e-1_f64) * t47713 - F::cast_from(0.47488888888888888888e-1_f64) * t47715 - F::cast_from(0.23744444444444444444e-1_f64) * t47717 - F::cast_from(0.47488888888888888888e-1_f64) * t47724 - F::cast_from(0.31659259259259259258e-1_f64) * t47730 + F::cast_from(0.11872222222222222222e-1_f64) * t47732 + F::cast_from(0.71233333333333333332e-1_f64) * t47738 - F::cast_from(0.79148148148148148147e-2_f64) * t41656;
    t59846
}
