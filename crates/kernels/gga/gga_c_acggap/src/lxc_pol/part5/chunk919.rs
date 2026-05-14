//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 919/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk919<F: Float>(t17039: F, t4737: F, t3379: F, t4963: F, t14230: F, t406: F, t1165: F, t1532: F, t3456: F, t1181: F, t3451: F, t360: F, t4183: F, t5096: F, t997: F, t3409: F, t4364: F) -> (F, F, F, F, F, F, F) {
    let t17040 = t17039 * t4737;
    let t17042 = t3379 * t4963;
    let t17056 = t14230 * t406;
    let t17059 = t3456 * t1165 * t1532 * t17056;
    let t17064 = t3451 * t1181 * t1532 * t4183 * t360;
    let t17066 = t997 * t5096;
    let t17068 = t3409 * t4364;
    (t17040, t17042, t17056, t17059, t17064, t17066, t17068)
}
