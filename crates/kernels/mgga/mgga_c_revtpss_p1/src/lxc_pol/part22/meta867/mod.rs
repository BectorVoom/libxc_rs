//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta867 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3023;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta867<F: Float>(t14325: F, t14370: F, t14322: F, t2626: F, t4398: F, t9425: F, t10555: F, t14613: F, t10565: F, t1532: F, t9419: F, t162: F, t40188: F, t14362: F, t9572: F, t37: F, t4391: F, t14767: F, t221: F, t10703: F, t2674: F, t2661: F, t2662: F, t2754: F, t4352: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50880, t50883, t50888, t50890, t50892, t50893, t50895) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3023::<F>(t14325, t14370, t14322, t2626, t4398, t9425, t10555, t14613, t10565, t1532, t9419, t162, t40188);
        let (t50901, t50903, t50933, t50937) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3024::<F>(t14362, t9572, t37, t4391, t14767, t221, t10703, t2674, t2661, t2662, t2754, t4352);
    (t50880, t50883, t50888, t50890, t50892, t50893, t50895, t50901, t50903, t50933, t50937)
}
