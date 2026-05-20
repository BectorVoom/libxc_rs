//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta371<F: Float>(t127: F, t3672: F, t371: F, t3671: F, t140: F, t3693: F, t1222: F, t1226: F, t697: F, t3688: F, t3700: F, t3367: F, t404: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12995, t12996, t12998, t12999, t13011, t13012, t13014, t13015, t13017, t13018, t13026) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1916::<F>(t127, t3672, t371, t3671, t140, t3693, t1222, t1226, t697, t3688, t3700, t3367, t404);
    (t12995, t12996, t12998, t12999, t13011, t13012, t13014, t13015, t13017, t13018, t13026)
}
