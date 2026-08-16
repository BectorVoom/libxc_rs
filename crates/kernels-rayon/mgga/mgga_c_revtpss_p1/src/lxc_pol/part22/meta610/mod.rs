//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2512;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2513;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta610(t19533: f64, t3318: f64, t3304: f64, t1043: f64, t16553: f64, t19450: f64, t1093: f64, t11788: f64, t12160: f64, t15655: f64, t16502: f64, t16544: f64, t16552: f64, t1685: f64, t19509: f64, t19512: f64, t19515: f64, t19521: f64, t19526: f64, t3204: f64, t3223: f64, t3299: f64, t3317: f64, t4857: f64, t4964: f64, t4967: f64, t4977: f64, t4981: f64, t4984: f64, t6235: f64, t6362: f64, t6371: f64, t6386: f64, t359: f64, t6343: f64, t999: f64, t1086: f64, t1647: f64, t4995: f64, t3153: f64, t6299: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19534, t19539, t19549, t19554) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2512(t19533, t3318, t3304, t1043, t16553, t19450, t1093, t11788, t12160, t15655, t16502, t16544, t16552, t1685, t19509, t19512, t19515, t19521, t19526, t3204, t3223, t3299, t3317, t4857, t4964, t4967, t4977, t4981, t4984, t6235, t6362, t6371, t6386);
        let (t19556, t19557, t19566, t19569, t19572) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2513(t359, t6343, t999, t1086, t6235, t1647, t4995, t3153, t6299);
    (t19534, t19539, t19549, t19554, t19556, t19557, t19566, t19569, t19572)
}
