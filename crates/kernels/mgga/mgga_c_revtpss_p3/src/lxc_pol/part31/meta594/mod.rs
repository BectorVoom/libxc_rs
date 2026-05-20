//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2021;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2022;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta594<F: Float>(t25953: F, t26072: F, t2435: F, t25913: F, t7289: F, t94600: F, t2028: F, t3999: F, t25875: F, t25894: F, t25877: F, t94382: F, t94590: F, t25304: F, t25949: F, t25946: F, t25878: F, t94661: F, t7246: F, t9692: F, t1419: F, t7063: F, t25898: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94756, t94758, t94761, t94763, t94768, t94771) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2021::<F>(t25953, t26072, t2435, t25913, t7289, t94600, t2028, t3999, t25875, t25894, t25877, t94382);
        let (t94772, t94777, t94779, t94784, t94801, t94802) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2022::<F>(t94590, t94771, t25304, t25949, t25946, t25878, t94661, t7246, t9692, t1419, t7063, t25898);
    (t94756, t94758, t94761, t94763, t94768, t94771, t94772, t94777, t94779, t94784, t94801, t94802)
}
