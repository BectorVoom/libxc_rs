//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2210;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2211;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta678<F: Float>(t108710: F, t1936: F, t21881: F, t93: F, t30143: F, t7002: F, t27123: F, t7741: F, t28219: F, t28042: F, t7889: F, t2322: F, t30004: F, t5523: F, t27833: F, t7935: F, t1448: F, t6922: F, t28196: F, t28197: F, t28067: F, t98450: F, t7897: F, t8995: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t109241, t109244, t109246, t109248, t109250, t109252, t109254) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2210::<F>(t108710, t1936, t21881, t93, t30143, t7002, t27123, t7741, t28219, t28042, t7889, t2322, t30004);
        let (t109256, t109262, t109266, t109268, t109269) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2211::<F>(t30004, t5523, t27833, t7935, t1448, t6922, t28196, t28197, t28067, t98450, t7897, t8995);
    (t109241, t109244, t109246, t109248, t109250, t109252, t109254, t109256, t109262, t109266, t109268, t109269)
}
