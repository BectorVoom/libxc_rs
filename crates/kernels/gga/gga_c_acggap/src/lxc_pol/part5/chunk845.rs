//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 845/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk845<F: Float>(t1149: F, t986: F, t1152: F, t1156: F, t3573: F, t1089: F, t175: F, t301: F, t3101: F, t384: F, t13690: F, t13693: F, t13726: F, t13736: F, t13745: F, t1161: F, t134: F, t3220: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13787 = t986 * t1149;
    let t13788 = t13787 * t1152;
    let t13791 = t3573 * t1156;
    let t13802 = t384 * t1089 * t175 * t3101 * t301;
    let t13804 = 35.0 / 9.0 * t13690;
    let t13805 = 130.0 / 27.0 * t13693;
    let t13810 = 35.0 / 36.0 * t13726;
    let t13812 = 910.0 / 81.0 * t13736;
    let t13814 = 100.0 / 9.0 * t13745;
    let t13850 = t1161 * t134 * t3220;
    (t13787, t13788, t13791, t13802, t13804, t13805, t13810, t13812, t13814, t13850)
}
