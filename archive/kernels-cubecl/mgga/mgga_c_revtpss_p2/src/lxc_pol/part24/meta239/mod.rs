//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta239<F: Float>(t2435: F, t5760: F, t1892: F, t3999: F, t225: F, t9990: F, t213: F, t2777: F, t5759: F, t2439: F, t136: F, t1883: F) -> (F, F, F, F, F, F, F) {
        let (t14166, t14171, t14192, t14193, t14202, t14203, t14219) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1000::<F>(t2435, t5760, t1892, t3999, t225, t9990, t213, t2777, t5759, t2439, t136, t1883);
    (t14166, t14171, t14192, t14193, t14202, t14203, t14219)
}
