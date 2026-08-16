//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 908/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk908<F: Float>(t38402: F, t38418: F, t38435: F, t38637: F, t488: F, t1852: F, t492: F, t8355: F, t1820: F, t102: F, t8416: F, t100: F) -> (F, F, F, F) {
    let t38640 = t488 * (t38402 + t38418 + t38435 + t38637);
    let t38645 = t1852 * t492 * t8355;
    let t38647 = t1820 * t1820;
    let t38648 = t1852 * t38647;
    let t38651 = F::cast_from(1.0_f64) / t8416 / t102;
    let t38652 = t100 * t38651;
    (t38640, t38645, t38648, t38652)
}
