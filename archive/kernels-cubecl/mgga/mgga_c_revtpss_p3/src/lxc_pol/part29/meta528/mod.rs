//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1856;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta528<F: Float>(t26703: F, t575: F, t26743: F, t571: F, t1455: F, t7560: F, t2110: F, t4168: F, t1923: F, t25146: F, t7348: F, t25150: F, t7349: F, t26169: F, t6954: F, t26204: F, t6977: F, t25117: F, t1927: F, t72: F, t843: F, t26205: F, t45958: F, t7342: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t95184, t95186, t95190, t95196, t95230, t95241) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1856::<F>(t26703, t575, t26743, t571, t1455, t7560, t2110, t4168, t1923, t25146, t7348, t25150, t7349);
        let (t95243, t95246, t95248, t95253, t95255, t95259) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1857::<F>(t26169, t6954, t1923, t26204, t6977, t25117, t7349, t1927, t72, t843, t26205, t45958, t7342);
    (t95184, t95186, t95190, t95196, t95230, t95241, t95243, t95246, t95248, t95253, t95255, t95259)
}
