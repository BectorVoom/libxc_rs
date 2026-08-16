//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta727 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2568;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta727(t2482: f64, t4000: f64, t596: f64, t10003: f64, t1412: f64, t3923: f64, t2661: f64, t9835: f64, t9934: f64, t9914: f64, t9918: f64, t221: f64, t4018: f64, t4019: f64, t9899: f64, t4059: f64, t9909: f64, t9812: f64, t9962: f64, t13845: f64, t46751: f64, t9818: f64, t13847: f64, t9819: f64, t9840: f64, t9958: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47216, t47218, t47221, t47223, t47227) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2568(t2482, t4000, t596, t10003, t1412, t3923, t2661, t9835, t9934, t9914, t9918, t221, t4018, t4019, t9899);
        let (t47229, t47231, t47235, t47239, t47245) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2569(t4059, t9909, t9812, t9962, t13845, t46751, t9818, t9835, t13847, t9819, t9840, t9958);
    (t47216, t47218, t47221, t47223, t47227, t47229, t47231, t47235, t47239, t47245)
}
