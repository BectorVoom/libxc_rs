//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1019/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1019(t3379: f64, t4447: f64, t1456: f64, t3670: f64, t1008: f64, t4728: f64, t1005: f64, t4625: f64, t1137: f64, t5184: f64, t3409: f64, t4402: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17291 = t3379 * t4447;
    let t17302 = t3670 * t1456;
    let t17304 = t1008 * t4728;
    let t17306 = t1005 * t4625;
    let t17308 = t1137 * t5184;
    let t17310 = t3409 * t4402;
    (t17291, t17302, t17304, t17306, t17308, t17310)
}
