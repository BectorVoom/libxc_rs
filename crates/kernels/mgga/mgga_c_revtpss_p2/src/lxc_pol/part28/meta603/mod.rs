//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2082;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta603<F: Float>(t18163: F, t7742: F, t28063: F, t4254: F, t1937: F, t75485: F, t18227: F, t6993: F, t27126: F, t7003: F, t25856: F, t7732: F, t26090: F, t7898: F, t1353: F, t28198: F, t25082: F, t28197: F, t27833: F, t7239: F, t28177: F, t7235: F, t28056: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97639, t97641, t97643, t97645, t97647, t97649) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2082::<F>(t18163, t7742, t28063, t4254, t1937, t75485, t18227, t6993, t27126, t7003, t25856, t7732);
        let (t97653, t97657, t97659, t97661, t97663) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2083::<F>(t26090, t7898, t1353, t28198, t25082, t28197, t27833, t7239, t28177, t7235, t28056, t4254);
    (t97639, t97641, t97643, t97645, t97647, t97649, t97653, t97657, t97659, t97661, t97663)
}
