//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2553;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2554;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta632(t20112: f64, t380: f64, t1043: f64, t1089: f64, t6343: f64, t1668: f64, t4930: f64, t16449: f64, t1651: f64, t4772: f64, t5004: f64, t20089: f64, t19829: f64, t19836: f64, t1024: f64, t1087: f64, t12146: f64, t12149: f64, t12154: f64, t15670: f64, t19608: f64, t19612: f64, t19617: f64, t19856: f64, t3204: f64, t3278: f64, t3287: f64, t342: f64, t381: f64, t4961: f64, t4999: f64, t6365: f64, t6379: f64, t6389: f64, t989: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20113, t20119, t20123, t20128, t20133, t20136) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2553(t20112, t380, t1043, t1089, t6343, t1668, t4930, t16449, t1651, t4772, t5004, t20089);
        let (t20139, t20146, t20149) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2554(t1089, t19829, t19836, t1024, t1087, t12146, t12149, t12154, t15670, t19608, t19612, t19617, t19856, t20113, t20119, t20123, t20128, t20133, t20136, t3204, t3278, t3287, t342, t381, t4961, t4999, t6365, t6379, t6389, t989);
    (t20113, t20119, t20123, t20128, t20133, t20136, t20139, t20146, t20149)
}
