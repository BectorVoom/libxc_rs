//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2628;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2629;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2630;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta667(t1248: f64, t3604: f64, t6688: f64, t3720: f64, t20266: f64, t5312: f64, t17475: f64, t20293: f64, t20318: f64, t5308: f64, t20310: f64, t20306: f64, t1260: f64, t6601: f64, t1222: f64, t1266: f64, t12784: f64, t12855: f64, t17437: f64, t5304: f64, t5309: f64, t5313: f64, t5373: f64, t5391: f64, t6640: f64, t1264: f64, t20272: f64, t247: f64, t5405: f64, t6429: f64, t3626: f64, t6425: f64, t1794: f64, t5245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21119, t21120, t21121, t21126, t21129, t21134, t21137, t21140) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2628(t1248, t3604, t6688, t3720, t20266, t5312, t17475, t20293, t20318, t5308, t20310, t20306);
        let (t21143, t21146) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2629(t1260, t6601, t1222, t1266, t12784, t12855, t17437, t21121, t21126, t21129, t21134, t21137, t21140, t5304, t5309, t5313, t5373, t5391, t6640);
        let (t21153, t21156, t21157, t21160, t21161, t21164) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2630(t1264, t20272, t247, t5405, t6429, t3626, t6425, t1794, t5245);
    (t21119, t21120, t21121, t21143, t21146, t21153, t21156, t21157, t21160, t21161, t21164)
}
