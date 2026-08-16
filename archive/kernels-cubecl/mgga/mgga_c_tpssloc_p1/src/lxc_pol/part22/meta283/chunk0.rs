//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1432/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1432<F: Float>(t13012: F, t4130: F, t2563: F, t4138: F, t4134: F, t9546: F, t118: F, t4119: F, t794: F, t2576: F, t225: F, t4266: F) -> (F, F, F, F, F, F) {
    let t13014 = F::cast_from(0.23333333333333333332e-1_f64) * t13012 * t4130;
    let t13020 = t2563 * t4138;
    let t13022 = t9546 * t4134;
    let t13025 = t118 * t794 * t4119;
    let t13027 = F::cast_from(0.16666666666666666666e-2_f64) * t2576 * t13025;
    let t13042 = t4266 * t225;
    (t13014, t13020, t13022, t13025, t13027, t13042)
}
