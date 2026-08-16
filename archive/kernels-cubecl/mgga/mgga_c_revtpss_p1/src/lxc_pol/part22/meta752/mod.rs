//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta752 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2825;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta752<F: Float>(t225: F, t42066: F, t41306: F, t3057: F, t3259: F, t367: F, t371: F, t373: F, t9291: F, t3197: F, t3201: F, t3231: F, t11773: F, t11865: F, t3205: F, t3206: F, t676: F, t2852: F, t3154: F, t2251: F, t1011: F, t3247: F, t697: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42067, t42078, t42107, t42121, t42124, t42141) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2825::<F>(t225, t42066, t41306, t3057, t3259, t367, t371, t373, t9291, t3197, t3201, t3231);
        let (t42155, t42176, t42215, t42216, t42254) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2826::<F>(t11773, t11865, t3205, t3206, t371, t676, t2852, t3154, t2251, t1011, t3247, t697);
    (t42067, t42078, t42107, t42121, t42124, t42141, t42155, t42176, t42215, t42216, t42254)
}
