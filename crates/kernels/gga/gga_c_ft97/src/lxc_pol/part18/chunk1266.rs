//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1266/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1266<F: Float>(t1286: F, t26129: F, t376: F, t22870: F, t6414: F, t102585: F, t102887: F, t103346: F, t11427: F, t1337: F, t1853: F, t22919: F, t25558: F, t25590: F, t25856: F, t3109: F, t38652: F, t38659: F, t492: F, t5748: F, t6547: F, t8418: F, t94067: F, t94070: F) -> (F,) {
    let t103972 = 2.0 / 9.0 * t1286 * t376 * t26129;
    let t103975 = t6414 * t22870;
    let t103981 = 8.0 * t102585 + 48.0 * t38652 * t6547 * t1853 - 24.0 * t38659 * t25856 - 24.0 * t8418 * t25590 * t492 - t11427 * t1337 - t25558 * t22919 / 9.0 + t103972 - 2.0 * t3109 * t5748 + 2.0 / 27.0 * t103975 - t94067 / 9.0 - t94070 / 18.0 + 4.0 * t102887 + 8.0 * t103346;
    (t103981,)
}
