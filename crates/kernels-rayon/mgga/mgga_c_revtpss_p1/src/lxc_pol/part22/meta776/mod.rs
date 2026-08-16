//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta776 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2865;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta776(t1235: f64, t3661: f64, t371: f64, t676: f64, t1236: f64, t2434: f64, t3671: f64, t3672: f64, t12625: f64, t458: f64, t456: f64, t225: f64, t43813: f64, t12984: f64, t3667: f64, t1261: f64, t12879: f64, t247: f64, t3372: f64, t3368: f64, t12881: f64, t3647: f64, t1224: f64, t12268: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44823, t44829, t44838, t44842, t44843) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2865(t1235, t3661, t371, t676, t1236, t2434, t3671, t3672, t12625, t458, t456, t225);
        let (t44865, t44884, t44902, t44906, t44917, t44919) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2866(t43813, t12984, t3667, t1261, t12879, t247, t3372, t3368, t12881, t3647, t1224, t12268);
    (t44823, t44829, t44838, t44842, t44843, t44865, t44884, t44902, t44906, t44917, t44919)
}
