//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2005;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2006;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta408<F: Float>(t5710: F, t72: F, t1432: F, t686: F, t136: F, t1892: F, t2457: F, t3964: F, t2435: F, t5760: F, t3999: F, t545: F, t869: F, t689: F, t225: F, t9990: F, t213: F, t10062: F, t10130: F, t13805: F, t1399: F, t14122: F, t14127: F, t1883: F, t3924: F, t4004: F, t4057: F, t5675: F, t5735: F, t5745: F, t5755: F, t5767: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14155, t14158, t14159, t14161, t14166, t14171, t14188) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2005::<F>(t5710, t72, t1432, t686, t136, t1892, t2457, t3964, t2435, t5760, t3999, t545);
        let (t14189, t14191, t14192, t14193) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2006::<F>(t14188, t869, t689, t225, t9990, t213);
        let t14200 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2007::<F>(t10062, t10130, t13805, t1399, t14122, t14127, t14158, t14161, t14166, t14171, t14191, t14193, t1883, t3924, t4004, t4057, t5675, t5735, t5745, t5755, t5767, t820);
    (t14155, t14158, t14159, t14161, t14166, t14171, t14188, t14189, t14191, t14192, t14193, t14200)
}
