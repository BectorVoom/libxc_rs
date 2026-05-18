//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1042/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1042<F: Float>(t3379: F, t5277: F, t5281: F, t1181: F, t15758: F, t3451: F, t535: F, t16325: F, t4282: F, t530: F, t1165: F, t3456: F, t4241: F, t4289: F) -> (F, F, F, F, F) {
    let t18017 = t3379 * t5277;
    let t18019 = t3379 * t5281;
    let t18027 = t3451 * t1181 * t535 * t15758;
    let t18031 = t4282 * t1181 * t530 * t16325;
    let t18035 = t3456 * t1165 * t4289 * t4241;
    (t18017, t18019, t18027, t18031, t18035)
}
