//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 944/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk944<F: Float>(t1163: F, t1165: F, t16548: F, t540: F, t12738: F, t5147: F, t1008: F, t5118: F, t1198: F, t1350: F, t384: F, t398: F, t4552: F, t997: F, t12572: F, t4488: F) -> (F, F, F, F, F, F) {
    let t18045 = t1163 * t1165 * t540 * t16548;
    let t18047 = t12738 * t5147;
    let t18062 = t1008 * t5118;
    let t18066 = t384 * t398 * t1198 * t1350;
    let t18072 = t997 * t4552;
    let t18079 = t12572 * t4488;
    (t18045, t18047, t18062, t18066, t18072, t18079)
}
