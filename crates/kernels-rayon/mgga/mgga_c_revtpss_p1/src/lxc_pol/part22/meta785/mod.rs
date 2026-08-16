//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta785 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta785(t1204: f64, t13141: f64, t3596: f64, t42859: f64, t460: f64, t1243: f64, t13126: f64, t12722: f64, t3566: f64, t5462: f64, t5477: f64, t1209: f64, t1284: f64, t3727: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45779, t45785, t45786, t45832, t45833, t45846, t45852, t45859, t45863, t45868) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2875(t1204, t13141, t3596, t42859, t460, t1243, t13126, t12722, t3566, t5462, t5477, t1209, t1284, t3727);
    (t45779, t45785, t45786, t45832, t45833, t45846, t45852, t45859, t45863, t45868)
}
