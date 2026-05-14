//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 937/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk937<F: Float>(t1089: F, t1421: F, t384: F, t966: F, t13087: F, t4908: F, t13263: F, t1562: F, t3379: F, t4701: F, t361: F, t435: F, t1441: F, t3237: F, t157: F, t3037: F) -> (F, F, F, F, F, F, F) {
    let t17729 = t384 * t1089 * t966 * t1421;
    let t17733 = t13087 * t4908;
    let t17740 = t13263 * t1562;
    let t17742 = t3379 * t4701;
    let t17752 = t361 * t435;
    let t17773 = t3237 * t1441;
    let t17775 = t157 * t3037;
    (t17729, t17733, t17740, t17742, t17752, t17773, t17775)
}
