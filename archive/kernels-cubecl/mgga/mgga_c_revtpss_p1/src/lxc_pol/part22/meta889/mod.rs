//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta889 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta889<F: Float>(t11409: F, t1621: F, t2968: F, t300: F, t3012: F, t11507: F, t15494: F, t11223: F, t379: F, t4930: F, t989: F, t11199: F, t1646: F) -> (F, F, F, F, F, F, F, F) {
        let (t52837, t52840, t52877, t52894, t52921, t52927, t52994, t53014) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3077::<F>(t11409, t1621, t2968, t300, t3012, t11507, t15494, t11223, t379, t4930, t989, t11199, t1646);
    (t52837, t52840, t52877, t52894, t52921, t52927, t52994, t53014)
}
