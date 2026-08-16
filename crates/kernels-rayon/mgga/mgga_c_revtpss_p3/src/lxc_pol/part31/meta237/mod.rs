//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1063;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1064;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1065;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1066;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta237(t1082: f64, t6244: f64, t1089: f64, t6271: f64, t1651: f64, t5004: f64, t6258: f64, t378: f64, t6305: f64, t3304: f64, t1668: f64, t1678: f64, t6299: f64, t3318: f64, t380: f64, t6343: f64, t1024: f64, t1087: f64, t1647: f64, t1685: f64, t1689: f64, t1692: f64, t3204: f64, t3287: f64, t3299: f64, t3317: f64, t342: f64, t381: f64, t4857: f64, t4954: f64, t6235: f64, t1079: f64, t1076: f64, t1652: f64, t1680: f64, t1696: f64, t3058: f64, t386: f64, t4747: f64, t4752: f64, t4778: f64, t4935: f64, t6245: f64, t6251: f64, t6259: f64, t6345: f64, t6351: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6362, t6365, t6368, t6371, t6374, t6375, t6379) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1063(t1082, t6244, t1089, t6271, t1651, t5004, t6258, t378, t6305, t3304, t1668, t1678);
        let (t6383, t6386, t6389, t6392) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1064(t1089, t378, t6299, t3318, t6374, t380, t6343, t1024, t1087, t1647, t1685, t1689, t1692, t3204, t3287, t3299, t3317, t342, t381, t4857, t4954, t6235, t6362, t6365, t6368, t6371, t6375, t6379);
        let t6393 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1065(t1079, t6392);
        let t6396 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1066(t1076, t1647, t1652, t1680, t1696, t3058, t342, t386, t4747, t4752, t4778, t4935, t6235, t6245, t6251, t6259, t6345, t6351, t6393, t995);
    (t6362, t6365, t6368, t6371, t6375, t6379, t6383, t6386, t6389, t6392, t6393, t6396)
}
