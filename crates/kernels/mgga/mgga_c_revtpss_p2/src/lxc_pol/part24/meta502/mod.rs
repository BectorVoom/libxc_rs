//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1507;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta502<F: Float>(t10726: F, t14586: F, t18408: F, t2661: F, t23334: F, t61625: F, t10850: F, t221: F, t23172: F, t2485: F, t23281: F, t2652: F, t10858: F, t23257: F, t23279: F, t10703: F, t2674: F, t2662: F, t6035: F, t61579: F, t1559: F, t18608: F) -> (F, F, F, F, F, F, F, F) {
        let (t76583, t76587, t76591, t76593) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1507::<F>(t10726, t14586, t18408, t2661, t23334, t61625, t10850, t221, t23172, t2485, t23281, t2652);
        let (t76596, t76615, t76619, t76645) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1508::<F>(t10858, t23257, t221, t23279, t10703, t2674, t2661, t2662, t6035, t61579, t1559, t18608);
    (t76583, t76587, t76591, t76593, t76596, t76615, t76619, t76645)
}
