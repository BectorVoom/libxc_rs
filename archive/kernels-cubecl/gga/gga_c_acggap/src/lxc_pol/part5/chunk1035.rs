//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1035/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1035<F: Float>(t1441: F, t3237: F, t157: F, t3037: F, t1165: F, t12731: F, t1532: F, t14047: F, t4273: F, t1106: F, t1181: F, t1567: F, t3361: F) -> (F, F, F, F, F) {
    let t17773 = t3237 * t1441;
    let t17775 = t157 * t3037;
    let t17778 = t12731 * t1165 * t1532 * t17775;
    let t17798 = t14047 * t4273;
    let t17804 = t3361 * t1181 * t1567 * t1106;
    (t17773, t17775, t17778, t17798, t17804)
}
