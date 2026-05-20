//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1494;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta403<F: Float>(t3191: F, t3201: F, t1021: F, t11970: F, t11874: F, t15688: F, t11714: F, t3111: F, t11722: F, t3106: F, t11727: F, t3223: F, t3230: F) -> (F, F, F, F, F, F, F) {
        let (t42324, t42326, t42328, t42334, t42336, t42338, t42340) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1494::<F>(t3191, t3201, t1021, t11970, t11874, t15688, t11714, t3111, t11722, t3106, t11727, t3223, t3230);
    (t42324, t42326, t42328, t42334, t42336, t42338, t42340)
}
