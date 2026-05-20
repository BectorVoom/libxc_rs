//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta707 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2730;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta707<F: Float>(t1868: F, t4003: F, t6843: F, t2723: F, t6016: F, t1544: F, t11660: F, t1469: F, t159: F, t2698: F, t1518: F, t648: F) -> (F, F, F, F, F, F, F) {
        let (t22841, t23037, t23160, t23334, t23898, t25273, t27123) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2730::<F>(t1868, t4003, t6843, t2723, t6016, t1544, t11660, t1469, t159, t2698, t1518, t648);
    (t22841, t23037, t23160, t23334, t23898, t25273, t27123)
}
