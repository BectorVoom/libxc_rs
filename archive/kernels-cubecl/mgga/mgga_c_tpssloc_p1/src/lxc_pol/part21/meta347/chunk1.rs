//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1746/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1746<F: Float>(t13012: F, t4130: F, t12971: F, t210: F, t214: F, t2563: F, t4138: F, t4134: F, t9546: F, t118: F, t4119: F, t794: F) -> (F, F, F, F, F) {
    let t13014 = F::cast_from(0.23333333333333333332e-1_f64) * t13012 * t4130;
    let t13017 = t210 * t214 * t12971;
    let t13020 = t2563 * t4138;
    let t13022 = t9546 * t4134;
    let t13025 = t118 * t794 * t4119;
    (t13014, t13017, t13020, t13022, t13025)
}
