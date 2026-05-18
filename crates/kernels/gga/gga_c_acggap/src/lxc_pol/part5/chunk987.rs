//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 987/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk987<F: Float>(t1140: F, t4434: F, t1101: F, t360: F, t1181: F, t3361: F, t540: F, t1165: F, t12816: F, t4417: F, t322: F, t368: F, t384: F, t398: F, t4875: F) -> (F, F, F, F, F) {
    let t16319 = t1140 * t4434;
    let t16325 = t1101 * t360;
    let t16328 = t3361 * t1181 * t540 * t16325;
    let t16332 = t3361 * t1165 * t4417 * t12816;
    let t16356 = t384 * t398 * t368 * t4875 * t322;
    (t16319, t16325, t16328, t16332, t16356)
}
