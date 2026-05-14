//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 926/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk926<F: Float>(t1456: F, t3670: F, t1008: F, t4728: F, t1005: F, t4625: F, t1137: F, t5184: F, t3409: F, t4402: F, t1165: F, t12991: F, t3044: F, t530: F, t5082: F, t935: F) -> (F, F, F, F, F, F, F) {
    let t17302 = t3670 * t1456;
    let t17304 = t1008 * t4728;
    let t17306 = t1005 * t4625;
    let t17308 = t1137 * t5184;
    let t17310 = t3409 * t4402;
    let t17314 = t12991 * t1165 * t530 * t3044;
    let t17316 = t935 * t5082;
    (t17302, t17304, t17306, t17308, t17310, t17314, t17316)
}
