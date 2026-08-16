//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta195 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1178;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta195<F: Float>(t4606: F, t916: F, t1600: F, t2897: F, t918: F, t923: F, t1606: F, t698: F, t2908: F, t4574: F, t141: F, t4579: F, t930: F, t4583: F, t2848: F, t2892: F, t2905: F, t2906: F, t4571: F, t4576: F, t4581: F, t4585: F, t4599: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4607, t4614, t4615, t4617, t4620, t4622, t4623, t4625) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1178::<F>(t4606, t916, t1600, t2897, t918, t923, t1606, t698, t2908, t4574, t141, t4579, t930);
        let (t4626, t4628, t4629, t4631) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1179::<F>(t141, t4625, t4583, t930, t2848, t2892, t2905, t2906, t4571, t4576, t4581, t4585, t4599, t4607, t4615, t4617, t4620, t4623);
    (t4607, t4614, t4615, t4617, t4620, t4622, t4623, t4625, t4626, t4628, t4629, t4631)
}
