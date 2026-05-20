//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta289<F: Float>(t3172: F, t6307: F, t3150: F, t4820: F, t4879: F, t11725: F, t247: F, t6092: F, t1063: F, t3109: F, t6100: F, t1647: F, t1678: F) -> (F, F, F, F, F, F, F, F) {
        let (t20029, t20030, t20034, t20050, t20051, t20054, t20055, t20175) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1070::<F>(t3172, t6307, t3150, t4820, t4879, t11725, t247, t6092, t1063, t3109, t6100, t1647, t1678);
    (t20029, t20030, t20034, t20050, t20051, t20054, t20055, t20175)
}
