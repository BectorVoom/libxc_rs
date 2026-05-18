//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 647/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk647<F: Float>(t1524: F, t540: F, t960: F, t1165: F, t1439: F, t4267: F, t1181: F, t1454: F, t1533: F, t5862: F, t1761: F, t3409: F) -> (F, F, F, F, F, F) {
    let t6319 = t540 * t1524;
    let t6320 = t960 * t6319;
    let t6324 = t1165 * t4267 * t1439;
    let t6328 = t1181 * t4267 * t1454;
    let t6332 = t1165 * t5862 * t1533;
    let t6335 = t3409 * t1761;
    (t6319, t6320, t6324, t6328, t6332, t6335)
}
