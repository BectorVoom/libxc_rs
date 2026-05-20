//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta768 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2568;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta768<F: Float>(t3599: F, t56802: F, t3609: F, t3623: F, t53739: F, t13127: F, t1214: F, t3611: F, t12831: F, t17395: F, t13148: F, t17728: F, t460: F, t489: F) -> (F, F, F, F, F, F, F, F) {
        let (t56803, t56806, t56878, t56879, t56947, t56953, t56997, t57005) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2568::<F>(t3599, t56802, t3609, t3623, t53739, t13127, t1214, t3611, t12831, t17395, t13148, t17728, t460, t489);
    (t56803, t56806, t56878, t56879, t56947, t56953, t56997, t57005)
}
