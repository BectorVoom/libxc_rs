//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1844;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta493<F: Float>(t25304: F, t7283: F, t25946: F, t25949: F, t786: F, t7286: F, t225: F, t26034: F, t1426: F, t3999: F, t26044: F, t4003: F, t213: F, t7274: F, t1445: F, t2027: F, t25921: F, t25961: F, t25966: F, t26036: F, t26040: F, t26043: F, t26046: F, t26051: F, t26055: F, t26058: F, t26062: F, t26065: F, t26067: F, t4078: F, t561: F, t7279: F, t7295: F, t7304: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t26069, t26071, t26072, t26073, t26075, t26079, t26080) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1844::<F>(t25304, t7283, t25946, t25949, t786, t7286, t225, t26034, t1426, t3999, t26044, t4003);
        let (t26081, t26084, t26087) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1845::<F>(t26079, t26080, t213, t7274, t1445, t2027, t25921, t25961, t25966, t26036, t26040, t26043, t26046, t26051, t26055, t26058, t26062, t26065, t26067, t26071, t26073, t26075, t4078, t561, t7279, t7295, t7304);
    (t26069, t26071, t26072, t26073, t26075, t26079, t26081, t26084, t26087)
}
