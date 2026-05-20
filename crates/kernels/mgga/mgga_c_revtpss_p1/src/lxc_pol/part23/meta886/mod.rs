//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta886 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2800;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta886<F: Float>(t2289: F, t5892: F, t21821: F, t625: F, t21824: F, t1455: F, t6951: F, t1464: F, t6936: F, t22571: F, t571: F, t25048: F, t575: F) -> (F, F, F, F, F, F, F) {
        let (t75639, t75641, t75643, t75720, t75727, t75796, t75808) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2800::<F>(t2289, t5892, t21821, t625, t21824, t1455, t6951, t1464, t6936, t22571, t571, t25048, t575);
    (t75639, t75641, t75643, t75720, t75727, t75796, t75808)
}
