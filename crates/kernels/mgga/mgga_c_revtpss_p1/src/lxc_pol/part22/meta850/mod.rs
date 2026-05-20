//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta850 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta850<F: Float>(t116: F, t13424: F, t10199: F, t1514: F, t2289: F, t4264: F, t13459: F, t625: F, t13462: F, t13510: F, t105: F, t4283: F, t588: F) -> (F, F, F, F, F, F, F) {
        let (t49686, t49698, t49700, t49702, t49704, t49724, t49745) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2990::<F>(t116, t13424, t10199, t1514, t2289, t4264, t13459, t625, t13462, t13510, t105, t4283, t588);
    (t49686, t49698, t49700, t49702, t49704, t49724, t49745)
}
