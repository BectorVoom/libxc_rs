//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 740/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk740<F: Float>(t1: F, t9735: F, t123: F, t4599: F, t4626: F, t6799: F, t6: F, t2024: F, t4623: F, t1256: F, t127: F, t2030: F, t4631: F, t4649: F, t6879: F, t4616: F, t6941: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13129 = t9735 * t1;
    let t13136 = t123 * t4599;
    let t13158 = t6799 * t4626;
    let t13160 = t6 * t4599;
    let t13174 = t4623 * t2024;
    let t13185 = t2024 * t1256;
    let t13190 = t127 * t1256;
    let t13202 = t2030 * t4631;
    let t13204 = t4649 * t2024;
    let t13209 = t4623 * t6879;
    let t13214 = t4623 * t127;
    let t13248 = t4649 * t127;
    let t13260 = t6941 * t4616;
    (t13129, t13136, t13158, t13160, t13174, t13185, t13190, t13202, t13204, t13209, t13214, t13248, t13260)
}
