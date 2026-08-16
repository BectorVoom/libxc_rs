//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2288;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta615(t24493: f64, t3523: f64, t1196: f64, t1179: f64, t1188: f64, t24407: f64, t1832: f64, t6752: f64, t1828: f64, t3737: f64, t6744: f64, t1774: f64, t1277: f64, t6702: f64, t13182: f64, t13100: f64, t24228: f64, t247: f64, t1794: f64, t6628: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24494, t24496, t24498, t24500, t24501, t24509, t24514) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2288(t24493, t3523, t1196, t1179, t1188, t24407, t1832, t6752, t1828, t3737, t6744, t1774);
        let (t24515, t24519, t24524, t24525, t24535, t24543) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2289(t1277, t24514, t1774, t3737, t6702, t1828, t13182, t13100, t24228, t247, t1794, t6628);
    (t24494, t24496, t24498, t24500, t24501, t24509, t24515, t24519, t24524, t24525, t24535, t24543)
}
