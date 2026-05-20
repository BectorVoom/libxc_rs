//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1632;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1633;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1634;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta433<F: Float>(t1261: F, t12944: F, t3172: F, t12932: F, t3711: F, t221: F, t461: F, t462: F, t624: F, t1250: F, t606: F, t1248: F, t2258: F, t1263: F, t3588: F, t372: F, t1222: F, t12809: F, t12855: F, t13069: F, t17344: F, t17351: F, t17354: F, t17693: F, t17694: F, t247: F, t3591: F, t3604: F, t3611: F, t3719: F, t3720: F, t3723: F, t43839: F, t44759: F, t44769: F, t44773: F, t44776: F, t44778: F, t44786: F, t5312: F, t1235: F, t3661: F, t371: F, t676: F, t1236: F, t2434: F, t1208: F, t12689: F, t225: F, t480: F, t3671: F, t3672: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t44789, t44792, t44797, t44800) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1632::<F>(t1261, t12944, t3172, t12932, t3711, t221, t461, t462, t624, t1250, t606, t1248, t2258);
        let (t44808, t44812) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1633::<F>(t1263, t3588, t372, t1222, t12809, t12855, t13069, t17344, t17351, t17354, t17693, t17694, t247, t3591, t3604, t3611, t3719, t3720, t3723, t43839, t44759, t44769, t44773, t44776, t44778, t44786, t44789, t44792, t44797, t44800, t5312);
        let (t44823, t44829, t44831, t44832, t44833, t44838) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1634::<F>(t1235, t3661, t371, t676, t1236, t2434, t1208, t12689, t225, t480, t3671, t3672);
    (t44800, t44808, t44812, t44823, t44829, t44831, t44832, t44833, t44838)
}
