//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta280<F: Float>(t18814: F, t689: F, t6042: F, t786: F, t789: F, t6049: F, t779: F, t14987: F, t4481: F, t6075: F, t892: F, t262: F, t5962: F) -> (F, F, F, F, F, F, F, F) {
        let (t18815, t18821, t18822, t18825, t18826, t18828, t18850, t18860) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1054::<F>(t18814, t689, t6042, t786, t789, t6049, t779, t14987, t4481, t6075, t892, t262, t5962);
    (t18815, t18821, t18822, t18825, t18826, t18828, t18850, t18860)
}
