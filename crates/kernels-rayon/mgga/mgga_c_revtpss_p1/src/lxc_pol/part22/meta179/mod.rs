//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta179 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1171;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta179(t1310: f64, t1518: f64, t1514: f64, t625: f64, t1513: f64, t2339: f64, t665: f64, t1504: f64, t2349: f64, t658: f64, t100: f64, t2: f64, t580: f64, t1509: f64, t2357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4257 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1171(t1310, t1518);
        let (t4261, t4263, t4264, t4269, t4270, t4273, t4274, t4279) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1172(t1514, t625, t1513, t2339, t665, t1504, t2349, t658, t100, t2, t580, t1509, t2357);
    (t4257, t4261, t4263, t4264, t4269, t4270, t4273, t4274, t4279)
}
