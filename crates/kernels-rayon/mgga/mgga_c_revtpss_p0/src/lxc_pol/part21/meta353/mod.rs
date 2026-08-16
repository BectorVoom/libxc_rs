//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1694;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta353(t140: f64, t3247: f64, t1011: f64, t3254: f64, t1015: f64, t10326: f64, t1012: f64, t3237: f64, t1014: f64, t2852: f64, t10356: f64, t245: f64, t3089: f64, t3088: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11752, t11753, t11755, t11756, t11758, t11759, t11762, t11763, t11766, t11767, t11772) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1694(t140, t3247, t1011, t3254, t1015, t10326, t1012, t3237, t1014, t2852, t10356, t245, t3089);
        let t11773 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1695(t11772, t3088);
    (t11752, t11753, t11755, t11756, t11758, t11759, t11762, t11763, t11766, t11767, t11772, t11773)
}
