//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 938/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk938<F: Float>(t8017: F, t898: F, t2295: F, t3135: F, t891: F, t2328: F, t3157: F, t3162: F, t237: F, t3113: F, t900: F, t2332: F, t3147: F, t7930: F, t6090: F, t6093: F, t6127: F, t7947: F, t7955: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8019 = 0.10389515463408878255e3 * t898 * t8017;
    let t8020 = t2295 * t3135;
    let t8021 = t8020 * t891;
    let t8023 = 0.23392894490538584828e1 * t898 * t8021;
    let t8025 = 0.11696447245269292414e1 * t2328 * t3157;
    let t8027 = 0.34631718211362927518e2 * t2328 * t3162;
    let t8028 = t237 * t3113;
    let t8030 = 0.11696447245269292414e1 * t8028 * t900;
    let t8034 = 0.11696447245269292414e1 * t3147 * t2332;
    let t8038 = 0.18541666666666666667e-1 * t7930;
    let t8040 = -t6127 + 0.24722222222222222222e-1 * t6090 - 0.92708333333333333333e-2 * t6093 + 0.12361111111111111111e-1 * t7955 - t8038 + 0.278125e-1 * t7947;
    (t8019, t8020, t8021, t8023, t8025, t8027, t8028, t8030, t8034, t8040)
}
