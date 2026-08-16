//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta752 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2825;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta752(t225: f64, t42066: f64, t41306: f64, t3057: f64, t3259: f64, t367: f64, t371: f64, t373: f64, t9291: f64, t3197: f64, t3201: f64, t3231: f64, t11773: f64, t11865: f64, t3205: f64, t3206: f64, t676: f64, t2852: f64, t3154: f64, t2251: f64, t1011: f64, t3247: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42067, t42078, t42107, t42121, t42124, t42141) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2825(t225, t42066, t41306, t3057, t3259, t367, t371, t373, t9291, t3197, t3201, t3231);
        let (t42155, t42176, t42215, t42216, t42254) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2826(t11773, t11865, t3205, t3206, t371, t676, t2852, t3154, t2251, t1011, t3247, t697);
    (t42067, t42078, t42107, t42121, t42124, t42141, t42155, t42176, t42215, t42216, t42254)
}
