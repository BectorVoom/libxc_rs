//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2623;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta664<F: Float>(t12712: F, t471: F, t6688: F, t3720: F, t1774: F, t3367: F, t4181: F, t3626: F, t6622: F, t73: F) -> (F, F, F, F, F, F, F) {
        let (t21028, t21029, t21030, t21035, t21036, t21037, t21040) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2623::<F>(t12712, t471, t6688, t3720, t1774, t3367, t4181, t3626, t6622, t73);
    (t21028, t21029, t21030, t21035, t21036, t21037, t21040)
}
