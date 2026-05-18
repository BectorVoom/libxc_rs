//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1154/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1154<F: Float>(t2001: F, t5816: F, t1988: F, t9577: F, t1095: F, t1980: F, t30058: F, t5655: F, t1967: F, t9531: F, t1901: F, t7614: F) -> (F, F, F, F, F) {
    let t39937 = t2001 * t5816;
    let t39939 = t1988 * t9577;
    let t39944 = t1980 * t30058 * t1095 * t5655;
    let t39946 = t1967 * t9531;
    let t39948 = t7614 * t1901;
    (t39937, t39939, t39944, t39946, t39948)
}
