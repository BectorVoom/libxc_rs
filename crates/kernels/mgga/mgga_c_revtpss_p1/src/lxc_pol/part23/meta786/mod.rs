//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta786 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2597;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2598;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta786<F: Float>(t18281: F, t189: F, t18555: F, t2619: F, t14341: F, t4311: F, t18562: F, t2516: F, t2496: F, t5825: F, t749: F, t4401: F, t606: F, t14369: F, t4186: F, t2439: F, t6041: F, t780: F, t785: F, t18821: F, t2471: F, t18814: F, t2435: F, t14476: F, t1580: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t61266, t61282, t61289, t61294, t61296, t61303, t61305) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2597::<F>(t18281, t189, t18555, t2619, t14341, t4311, t18562, t2516, t2496, t5825, t749, t4401, t606);
        let (t61315, t61324, t61330, t61337, t61344) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2598::<F>(t14369, t4186, t4401, t2439, t6041, t780, t785, t18821, t2471, t18814, t2435, t14476, t1580, t689);
    (t61266, t61282, t61289, t61294, t61296, t61303, t61305, t61315, t61324, t61330, t61337, t61344)
}
