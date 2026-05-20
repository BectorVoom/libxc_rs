//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta837 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3137;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta837<F: Float>(t12248: F, t1732: F, t12415: F, t12222: F, t5192: F, t1196: F, t45289: F, t5205: F, t12235: F, t16673: F, t3531: F, t12361: F, t16655: F, t16658: F, t44101: F, t12243: F, t16665: F, t16669: F, t44012: F, t3384: F, t3427: F, t5105: F, t12571: F, t5198: F, t12485: F, t3524: F, t5180: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t57820, t57822, t57825, t57827, t57829, t57831) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3137::<F>(t12248, t1732, t12415, t12222, t5192, t1196, t45289, t5205, t12235, t16673, t3531, t12361, t16655);
        let (t57833, t57835, t57837, t57840, t57842, t57846) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3138::<F>(t16658, t44101, t12243, t16665, t16669, t44012, t3384, t3427, t5105, t12571, t5198, t1196, t12485, t3524, t5180);
    (t57820, t57822, t57825, t57827, t57829, t57831, t57833, t57835, t57837, t57840, t57842, t57846)
}
