//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta928 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3153;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta928<F: Float>(t12772: F, t17639: F, t3625: F, t17645: F, t1284: F, t17288: F, t3624: F, t12917: F, t17401: F, t17396: F, t1260: F, t17289: F, t17544: F, t3708: F, t12915: F, t16771: F, t247: F, t5384: F, t17763: F, t3636: F, t13085: F, t5391: F, t12881: F, t5381: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t57026, t57029, t57040, t57045, t57049, t57053) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3153::<F>(t12772, t17639, t3625, t17645, t1284, t17288, t3624, t12917, t17401, t17396, t1260, t17289);
        let (t57063, t57070, t57075, t57077, t57094) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3154::<F>(t17544, t3708, t12915, t16771, t247, t5384, t17763, t3636, t13085, t5391, t12881, t5381);
    (t57026, t57029, t57040, t57045, t57049, t57053, t57063, t57070, t57075, t57077, t57094)
}
