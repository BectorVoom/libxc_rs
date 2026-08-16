//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta918 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3127;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta918(t12046: f64, t1647: f64, t16551: f64, t989: f64, t12153: f64, t4746: f64, t16237: f64, t359: f64, t15654: f64, t3286: f64, t16543: f64, t3046: f64, t4995: f64, t15669: f64, t1651: f64, t378: f64, t342: f64, t43400: f64, t1086: f64, t15886: f64, t3057: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t55599, t55632, t55646, t55649, t55685, t55701) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3127(t12046, t1647, t16551, t989, t12153, t4746, t16237, t359, t15654, t3286, t16543, t3046);
        let (t55732, t55747, t55764, t55805, t55868, t55887) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3128(t4746, t4995, t15669, t3286, t1651, t378, t342, t43400, t1086, t15886, t16543, t3057);
    (t55599, t55632, t55646, t55649, t55685, t55701, t55732, t55747, t55764, t55805, t55868, t55887)
}
