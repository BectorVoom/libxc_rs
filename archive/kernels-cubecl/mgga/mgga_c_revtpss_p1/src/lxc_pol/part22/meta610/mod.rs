//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2512;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2513;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta610<F: Float>(t19533: F, t3318: F, t3304: F, t1043: F, t16553: F, t19450: F, t1093: F, t11788: F, t12160: F, t15655: F, t16502: F, t16544: F, t16552: F, t1685: F, t19509: F, t19512: F, t19515: F, t19521: F, t19526: F, t3204: F, t3223: F, t3299: F, t3317: F, t4857: F, t4964: F, t4967: F, t4977: F, t4981: F, t4984: F, t6235: F, t6362: F, t6371: F, t6386: F, t359: F, t6343: F, t999: F, t1086: F, t1647: F, t4995: F, t3153: F, t6299: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19534, t19539, t19549, t19554) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2512::<F>(t19533, t3318, t3304, t1043, t16553, t19450, t1093, t11788, t12160, t15655, t16502, t16544, t16552, t1685, t19509, t19512, t19515, t19521, t19526, t3204, t3223, t3299, t3317, t4857, t4964, t4967, t4977, t4981, t4984, t6235, t6362, t6371, t6386);
        let (t19556, t19557, t19566, t19569, t19572) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2513::<F>(t359, t6343, t999, t1086, t6235, t1647, t4995, t3153, t6299);
    (t19534, t19539, t19549, t19554, t19556, t19557, t19566, t19569, t19572)
}
