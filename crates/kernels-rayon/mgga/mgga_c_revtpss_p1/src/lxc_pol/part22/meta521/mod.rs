//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2292;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2293;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta521(t1757: f64, t3515: f64, t3497: f64, t5184: f64, t3523: f64, t5180: f64, t1187: f64, t12429: f64, t12470: f64, t12481: f64, t12486: f64, t12491: f64, t16955: f64, t16959: f64, t16962: f64, t16966: f64, t16971: f64, t16974: f64, t16979: f64, t3477: f64, t3496: f64, t3521: f64, t5163: f64, t5185: f64, t12555: f64, t1756: f64, t16710: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12397: f64, t16706: f64, t16708: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16982, t16985, t16988, t16989, t16992, t16995) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2292(t1757, t3515, t3497, t5184, t3523, t5180, t1187, t12429, t12470, t12481, t12486, t12491, t16955, t16959, t16962, t16966, t16971, t16974, t16979, t3477, t3496, t3521, t5163, t5185);
        let (t16997, t16998, t17010, t17011, t17020) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2293(t12555, t1756, t3497, t16710, t16712, t12297, t12299, t12301, t12303, t12397, t16706, t16708, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
    (t16982, t16985, t16988, t16989, t16992, t16995, t16997, t16998, t17010, t17011, t17020)
}
