//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta530<F: Float>(t1234: F, t24680: F, t1222: F, t140: F, t24826: F, t1209: F, t24864: F, t473: F, t24704: F, t3153: F, t13045: F, t6622: F) -> (F, F, F, F, F, F) {
        let (t84185, t84195, t84315, t84429, t84487, t84636) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1566::<F>(t1234, t24680, t1222, t140, t24826, t1209, t24864, t473, t24704, t3153, t13045, t6622);
    (t84185, t84195, t84315, t84429, t84487, t84636)
}
