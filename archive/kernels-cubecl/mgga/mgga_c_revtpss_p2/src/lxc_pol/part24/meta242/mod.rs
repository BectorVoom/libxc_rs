//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta242<F: Float>(t14362: F, t2630: F, t1469: F, t2609: F, t706: F, t1568: F, t785: F, t780: F, t2439: F, t1579: F, t2769: F, t2470: F, t4480: F) -> (F, F, F, F, F, F, F, F) {
        let (t14363, t14440, t14441, t14472, t14473, t14474, t14480, t14485) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1004::<F>(t14362, t2630, t1469, t2609, t706, t1568, t785, t780, t2439, t1579, t2769, t2470, t4480);
    (t14363, t14440, t14441, t14472, t14473, t14474, t14480, t14485)
}
