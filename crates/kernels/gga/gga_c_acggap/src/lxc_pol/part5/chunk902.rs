//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 902/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk902<F: Float>(t13092: F, t4732: F, t1181: F, t3491: F, t4282: F, t535: F, t4912: F, t1165: F, t12991: F, t3655: F, t4267: F, t13087: F, t4277: F, t4528: F, t997: F, t1008: F, t4542: F) -> (F, F, F, F, F, F, F) {
    let t16409 = t13092 * t4732;
    let t16415 = t4282 * t1181 * t535 * t3491;
    let t16417 = t13092 * t4912;
    let t16421 = t12991 * t1165 * t4267 * t3655;
    let t16423 = t13087 * t4277;
    let t16425 = t997 * t4528;
    let t16427 = t1008 * t4542;
    (t16409, t16415, t16417, t16421, t16423, t16425, t16427)
}
