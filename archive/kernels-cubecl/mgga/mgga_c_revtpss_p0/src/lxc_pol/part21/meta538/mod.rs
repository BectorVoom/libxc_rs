//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta538<F: Float>(t3379: F, t5105: F, t12327: F, t1723: F, t3391: F, t12331: F, t3390: F, t5079: F, t1134: F, t3399: F, t5071: F, t3407: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16846, t16851, t16852, t16854, t16855, t16857, t16858, t16860, t16862) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2198::<F>(t3379, t5105, t12327, t1723, t3391, t12331, t3390, t5079, t1134, t3399, t5071, t3407);
    (t16846, t16851, t16852, t16854, t16855, t16857, t16858, t16860, t16862)
}
