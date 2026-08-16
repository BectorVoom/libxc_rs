//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2555;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2556;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta633(t19508: f64, t19554: f64, t19606: f64, t20149: f64, t1079: f64, t20112: f64, t225: f64, t385: f64, t1096: f64, t6392: f64, t3269: f64, t1647: f64, t1678: f64, t378: f64, t6235: f64, t1076: f64, t1097: f64, t11187: f64, t16340: f64, t16374: f64, t1652: f64, t16597: f64, t1696: f64, t19856: f64, t3264: f64, t342: f64, t386: f64, t4778: f64, t4932: f64, t4941: f64, t6245: f64, t6345: f64, t6351: f64, t989: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t20151, t20152, t20168, t20172, t20175) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2555(t19508, t19554, t19606, t20149, t1079, t20112, t225, t385, t1096, t6392, t3269, t1647, t1678);
        let (t20178, t20187) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2556(t378, t6235, t1076, t1097, t11187, t16340, t16374, t1647, t1652, t16597, t1696, t19856, t20152, t20168, t20172, t20175, t3264, t342, t386, t4778, t4932, t4941, t6245, t6345, t6351, t989);
    (t20151, t20152, t20168, t20172, t20175, t20178, t20187)
}
