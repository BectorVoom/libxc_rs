//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 787/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk787<F: Float>(t2100: F, t30044: F, t7538: F, t7544: F, t1004: F, t1979: F, t7548: F, t137: F, t3101: F, t1089: F, t1095: F, t2079: F, t19: F, t3220: F, t336: F, t151: F, t177: F, t3558: F, t587: F) -> (F, F, F, F, F, F, F, F) {
    let t30045 = t30044 * t2100;
    let t30047 = t7538 * t7544;
    let t30049 = t1004 * t1979;
    let t30050 = t30049 * t7548;
    let t30052 = t137 * t3101;
    let t30055 = t2079 * t1089 * t1095 * t30052;
    let t30058 = t3220 * t19 * t336;
    let t30077 = t151 * t587 * t3558 * t177;
    (t30045, t30047, t30049, t30050, t30052, t30055, t30058, t30077)
}
