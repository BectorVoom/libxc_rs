//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2463;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2464;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2465;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2466;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta589<F: Float>(t11150: F, t5819: F, t606: F, t2850: F, t128: F, t4186: F, t4573: F, t6093: F, t689: F, t6097: F, t6092: F, t904: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18908, t18909, t18910, t18911) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2463::<F>(t11150, t5819, t606, t2850, t128);
        let (t18913, t18914, t18915) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2464::<F>(t4186, t4573, t2850, t128);
        let t18919 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2465::<F>(t6093, t689);
        let t18924 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2466::<F>(t6097, t689);
        let (t18926, t18927, t18928) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2467::<F>(t606, t6092, t904, t128);
    (t18908, t18909, t18910, t18911, t18913, t18914, t18915, t18919, t18924, t18926, t18927, t18928)
}
