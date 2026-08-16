//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1002/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1002<F: Float>(t3775: F, t4349: F, t1423: F, t3244: F, t1429: F, t3228: F, t1163: F, t1181: F, t1579: F, t4210: F, t16020: F, t535: F) -> (F, F, F, F, F) {
    let t16801 = t3775 * t4349;
    let t16803 = t3244 * t1423;
    let t16805 = t3228 * t1429;
    let t16814 = t1163 * t1181 * t1579 * t4210;
    let t16818 = t1163 * t1181 * t535 * t16020;
    (t16801, t16803, t16805, t16814, t16818)
}
