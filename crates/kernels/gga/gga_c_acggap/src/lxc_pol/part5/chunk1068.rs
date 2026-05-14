//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1068/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1068<F: Float>(t12586: F, t6184: F, t3382: F, t6148: F, t12589: F, t5940: F, t1008: F, t5975: F, t301: F, t5506: F, t1734: F, t839: F, t6361: F, t1163: F, t1166: F, t20417: F) -> (F, F, F, F, F, F, F, F) {
    let t21607 = t12586 * t6184;
    let t21609 = t3382 * t6148;
    let t21611 = t12589 * t5940;
    let t21613 = t1008 * t5975;
    let t21615 = t5506 * t301;
    let t21620 = t1734 * t839;
    let t21625 = t1008 * t6361;
    let t21632 = t1163 * t20417 * t1166;
    (t21607, t21609, t21611, t21613, t21615, t21620, t21625, t21632)
}
