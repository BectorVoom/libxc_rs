//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1571;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1572;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1573;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta309<F: Float>(t124: F, t836: F, t10779: F, t2749: F, t10777: F, t125: F, t2722: F, t2723: F, t775: F, t2747: F, t10730: F, t10734: F, t10737: F, t10742: F, t10746: F, t10749: F, t10752: F, t10756: F, t10758: F, t10762: F, t10766: F, t10773: F, t2730: F, t2745: F, t4362: F, t851: F, t2645: F, t4364: F, t4366: F, t837: F, t820: F, t823: F, t844: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10780, t10782, t10783, t10785, t10786) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1571::<F>(t124, t836, t10779, t2749, t10777, t125, t2722, t2723, t775);
        let (t10788, t10791) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1572::<F>(t10785, t10786, t2747, t10730, t10734, t10737, t10742, t10746, t10749, t10752, t10756, t10758, t10762, t10766, t10773, t10783, t2730, t2745, t4362, t851);
        let (t10794, t10799, t10803, t10807, t10811) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1573::<F>(t10785, t2747, t2749, t125, t2645, t4364, t4366, t837, t820, t823, t844);
    (t10780, t10782, t10783, t10786, t10788, t10791, t10794, t10799, t10803, t10807, t10811)
}
