//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta194 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1232;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1233;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1234;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta194<F: Float>(t2908: F, t4574: F, t141: F, t4579: F, t930: F, t4583: F, t2848: F, t2892: F, t2905: F, t2906: F, t4571: F, t4576: F, t4581: F, t4585: F, t4599: F, t4607: F, t4615: F, t4617: F, t4620: F, t935: F, t915: F, t1609: F, t2926: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4622, t4623, t4625, t4626, t4628, t4629, t4631) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1232::<F>(t2908, t4574, t141, t4579, t930, t4583, t2848, t2892, t2905, t2906, t4571, t4576, t4581, t4585, t4599, t4607, t4615, t4617, t4620);
        let t4632 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1233::<F>(t4631, t935);
        let (t4634, t4635) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1234::<F>(t4632, t915, t1609, t2926);
    (t4622, t4623, t4625, t4626, t4628, t4629, t4631, t4632, t4634, t4635)
}
