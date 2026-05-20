//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta865 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2758;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta865<F: Float>(t13845: F, t13847: F, t5675: F, t73731: F, t3938: F, t9816: F, t9818: F, t13848: F, t5659: F, t22159: F, t48836: F, t22120: F, t9962: F, t46917: F, t6871: F, t22298: F, t48862: F, t48863: F, t22098: F, t22102: F, t46740: F, t22299: F, t22295: F, t22111: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t73734, t73738, t73742, t73744, t73750) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2758::<F>(t13845, t13847, t5675, t73731, t3938, t9816, t9818, t13848, t5659, t22159, t48836, t22120, t9962);
        let (t73778, t73781, t73787, t73789, t73798, t73800, t73803) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2759::<F>(t46917, t6871, t22298, t48862, t48863, t22098, t9962, t22102, t46740, t22299, t22295, t22111);
    (t73734, t73738, t73742, t73744, t73750, t73778, t73781, t73787, t73789, t73798, t73800, t73803)
}
