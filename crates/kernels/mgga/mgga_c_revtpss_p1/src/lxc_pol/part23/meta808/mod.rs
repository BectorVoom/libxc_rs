//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta808 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2642;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2643;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta808<F: Float>(t18784: F, t2465: F, t686: F, t72: F, t4481: F, t51276: F, t6042: F, t786: F, t867: F, t2467: F, t14480: F, t252: F, t2782: F, t4533: F, t14991: F, t50208: F, t14485: F, t14987: F, t18657: F, t213: F, t14983: F, t18392: F, t262: F, t18838: F, t2411: F, t18969: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t63062, t63064, t63084, t63085, t63091) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2642::<F>(t18784, t2465, t686, t72, t4481, t51276, t6042, t786, t867, t2467, t14480, t252, t2782, t4533);
        let (t63094, t63099, t63103, t63109, t63146, t63160, t63240) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2643::<F>(t14991, t50208, t14485, t14987, t18657, t213, t14983, t18392, t262, t18838, t2411, t18969, t698);
    (t63062, t63064, t63084, t63085, t63091, t63094, t63099, t63103, t63109, t63146, t63160, t63240)
}
