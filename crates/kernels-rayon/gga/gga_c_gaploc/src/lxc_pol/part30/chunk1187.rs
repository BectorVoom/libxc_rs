//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1187/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1187(t31878: f64, t10185: f64, t29874: f64, t10257: f64, t3818: f64, t20896: f64, t2268: f64, t7937: f64, t2325: f64, t25556: f64, t882: f64, t883: f64) -> (f64, f64, f64, f64, f64) {
    let t31879 = 0.63233348079280332443e-2_f64 * t31878;
    let t31880 = t29874 * t10185;
    let t31881 = 0.47425011059460249332e-2_f64 * t31880;
    let t31883 = 0.15176003539027279786e0_f64 * t3818 * t10257;
    let t31886 = 0.34146007962811379518e0_f64 * t2268 * t7937 * t20896;
    let t31889 = t882 * t2325 * t883 * t25556;
    (t31879, t31881, t31883, t31886, t31889)
}
