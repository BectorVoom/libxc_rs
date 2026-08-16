//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1045/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1045<F: Float>(t13200: F, t3210: F, t13199: F, t2816: F, t5026: F, t1092: F, t2825: F, t4995: F, t4994: F, t1023: F, t13181: F, t1020: F) -> (F, F, F, F) {
    let t13201 = t3210 * t13200;
    let t13202 = t13199 * t13201;
    let t13204 = t5026 * t2816;
    let t13205 = t1092 * t13204;
    let t13207 = t2825 * t4995;
    let t13208 = t4994 * t13207;
    let t13210 = t13181 * t1023;
    let t13211 = t1020 * t13210;
    (t13202, t13205, t13208, t13211)
}
