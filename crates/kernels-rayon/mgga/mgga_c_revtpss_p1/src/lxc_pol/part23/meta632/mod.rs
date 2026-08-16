//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2326;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2327;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2328;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta632(t1501: f64, t1518: f64, t10208: f64, t69: f64, t26: f64, t65: f64, t1651: f64, t385: f64, t1774: f64, t494: f64, t9163: f64, t99: f64, t107: f64, t9232: f64, t5672: f64, t828: f64, t4363: f64, t2565: f64, t702: f64, t9305: f64, t2576: f64, t2585: f64, t9274: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30138, t31035, t33127, t33754, t34934, t36227) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2326(t1501, t1518, t10208, t69, t26, t65, t1651, t385, t1774, t494, t9163, t99);
        let (t36415, t36776, t36833, t39419) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2327(t107, t9232, t5672, t828, t4363, t2565, t702, t9305);
        let t39422 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2328(t2576, t2585, t9274);
    (t30138, t31035, t33127, t33754, t34934, t36227, t36415, t36776, t36833, t39419, t39422)
}
