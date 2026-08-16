//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1253/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1253(t5821: f64, t997: f64, t5811: f64, t5546: f64, t14056: f64, t6140: f64, t3391: f64, t4680: f64, t6143: f64, t1181: f64, t1432: f64, t15995: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23063 = t997 * t5821;
    let t23065 = t997 * t5811;
    let t23068 = t997 * t5546;
    let t23070 = t14056 * t6140;
    let t23077 = t3391 * t4680 * t6143;
    let t23081 = t3391 * t1181 * t15995 * t1432;
    (t23063, t23065, t23068, t23070, t23077, t23081)
}
