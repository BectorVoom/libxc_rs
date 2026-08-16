//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1844;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta493(t25304: f64, t7283: f64, t25946: f64, t25949: f64, t786: f64, t7286: f64, t225: f64, t26034: f64, t1426: f64, t3999: f64, t26044: f64, t4003: f64, t213: f64, t7274: f64, t1445: f64, t2027: f64, t25921: f64, t25961: f64, t25966: f64, t26036: f64, t26040: f64, t26043: f64, t26046: f64, t26051: f64, t26055: f64, t26058: f64, t26062: f64, t26065: f64, t26067: f64, t4078: f64, t561: f64, t7279: f64, t7295: f64, t7304: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26069, t26071, t26072, t26073, t26075, t26079, t26080) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1844(t25304, t7283, t25946, t25949, t786, t7286, t225, t26034, t1426, t3999, t26044, t4003);
        let (t26081, t26084, t26087) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1845(t26079, t26080, t213, t7274, t1445, t2027, t25921, t25961, t25966, t26036, t26040, t26043, t26046, t26051, t26055, t26058, t26062, t26065, t26067, t26071, t26073, t26075, t4078, t561, t7279, t7295, t7304);
    (t26069, t26071, t26072, t26073, t26075, t26079, t26081, t26084, t26087)
}
