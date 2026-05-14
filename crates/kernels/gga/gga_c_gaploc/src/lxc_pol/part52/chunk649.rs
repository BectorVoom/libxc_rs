//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 649/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk649<F: Float>(t13053: F, t13636: F, t13639: F, t13643: F, t13646: F, t13650: F, t13653: F, t13655: F, t13656: F, t13660: F, t13893: F, t13895: F, t13143: F, t13151: F, t13679: F, t13681: F, t13693: F, t13695: F, t13697: F, t13700: F, t13703: F, t13704: F, t13898: F, t13899: F) -> (F, F) {
    let t14402 = -0.57514388930881124514e0 * t13636 + 0.9585731488480187419e0 * t13639 - t13643 - t13646 - t13650 + t13653 + 0.38342925953920749676e1 * t13053 - t13655 - t13656 + t13660 - 0.59584149919750711116e-1 * t13893 + 0.59584149919750711116e-1 * t13895;
    let t14406 = t13679 + t13681 - t13693 - t13695 + t13697 - t13898 + t13899 + t13700 + t13703 - 0.44688112439813033337e-1 * t13704 + 0.63904876589867916127e-1 * t13143 - 0.63904876589867916127e-1 * t13151;
    (t14402, t14406)
}
