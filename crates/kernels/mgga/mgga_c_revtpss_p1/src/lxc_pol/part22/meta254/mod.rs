//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta254 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1570;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1571;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1572;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1573;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta254<F: Float>(t1082: F, t6244: F, t1089: F, t6271: F, t1651: F, t5004: F, t6258: F, t378: F, t6305: F, t3304: F, t1668: F, t1678: F, t6299: F, t3318: F, t380: F, t6343: F, t1024: F, t1087: F, t1647: F, t1685: F, t1689: F, t1692: F, t3204: F, t3287: F, t3299: F, t3317: F, t342: F, t381: F, t4857: F, t4954: F, t6235: F, t1079: F, t1076: F, t1652: F, t1680: F, t1696: F, t3058: F, t386: F, t4747: F, t4752: F, t4778: F, t4935: F, t6245: F, t6251: F, t6259: F, t6345: F, t6351: F, t995: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6362, t6365, t6368, t6371, t6374, t6375, t6379) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1570::<F>(t1082, t6244, t1089, t6271, t1651, t5004, t6258, t378, t6305, t3304, t1668, t1678);
        let (t6383, t6386, t6389, t6392) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1571::<F>(t1089, t378, t6299, t3318, t6374, t380, t6343, t1024, t1087, t1647, t1685, t1689, t1692, t3204, t3287, t3299, t3317, t342, t381, t4857, t4954, t6235, t6362, t6365, t6368, t6371, t6375, t6379);
        let t6393 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1572::<F>(t1079, t6392);
        let t6396 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1573::<F>(t1076, t1647, t1652, t1680, t1696, t3058, t342, t386, t4747, t4752, t4778, t4935, t6235, t6245, t6251, t6259, t6345, t6351, t6393, t995);
    (t6362, t6365, t6368, t6371, t6375, t6379, t6383, t6386, t6389, t6392, t6393, t6396)
}
