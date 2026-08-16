//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 671/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk671<F: Float>(t1022: F, t4772: F, t1096: F, t1092: F, t1768: F, t3178: F, t1709: F, t2811: F, t1008: F, t1014: F, t1750: F, t1126: F, t1749: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4773 = t1022 * t4772;
    let t4774 = t1096 * t4773;
    let t4775 = t1092 * t4774;
    let t4778 = t3178 * t1768;
    let t4779 = t1092 * t4778;
    let t4781 = t1709 * t2811;
    let t4782 = t4781 * t1008;
    let t4787 = t1014 * t1750;
    let t4789 = t1749 * t1126;
    (t4773, t4774, t4775, t4778, t4779, t4781, t4782, t4787, t4789)
}
