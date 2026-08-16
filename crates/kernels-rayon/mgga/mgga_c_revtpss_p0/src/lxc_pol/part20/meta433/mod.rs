//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1632;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1633;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1634;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta433(t1261: f64, t12944: f64, t3172: f64, t12932: f64, t3711: f64, t221: f64, t461: f64, t462: f64, t624: f64, t1250: f64, t606: f64, t1248: f64, t2258: f64, t1263: f64, t3588: f64, t372: f64, t1222: f64, t12809: f64, t12855: f64, t13069: f64, t17344: f64, t17351: f64, t17354: f64, t17693: f64, t17694: f64, t247: f64, t3591: f64, t3604: f64, t3611: f64, t3719: f64, t3720: f64, t3723: f64, t43839: f64, t44759: f64, t44769: f64, t44773: f64, t44776: f64, t44778: f64, t44786: f64, t5312: f64, t1235: f64, t3661: f64, t371: f64, t676: f64, t1236: f64, t2434: f64, t1208: f64, t12689: f64, t225: f64, t480: f64, t3671: f64, t3672: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44789, t44792, t44797, t44800) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1632(t1261, t12944, t3172, t12932, t3711, t221, t461, t462, t624, t1250, t606, t1248, t2258);
        let (t44808, t44812) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1633(t1263, t3588, t372, t1222, t12809, t12855, t13069, t17344, t17351, t17354, t17693, t17694, t247, t3591, t3604, t3611, t3719, t3720, t3723, t43839, t44759, t44769, t44773, t44776, t44778, t44786, t44789, t44792, t44797, t44800, t5312);
        let (t44823, t44829, t44831, t44832, t44833, t44838) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1634(t1235, t3661, t371, t676, t1236, t2434, t1208, t12689, t225, t480, t3671, t3672);
    (t44800, t44808, t44812, t44823, t44829, t44831, t44832, t44833, t44838)
}
