//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1000/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1000<F: Float>(t8013: F, t898: F, t1208: F, t6121: F, t2321: F, t2295: F, t3135: F, t891: F, t2328: F, t3157: F, t3162: F, t237: F, t3113: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8015 = F::cast_from(0.34631718211362927518e2_f64) * t898 * t8013;
    let t8016 = t6121 * t1208;
    let t8017 = t8016 * t2321;
    let t8019 = F::cast_from(0.10389515463408878255e3_f64) * t898 * t8017;
    let t8020 = t2295 * t3135;
    let t8021 = t8020 * t891;
    let t8023 = F::cast_from(0.23392894490538584828e1_f64) * t898 * t8021;
    let t8025 = F::cast_from(0.11696447245269292414e1_f64) * t2328 * t3157;
    let t8027 = F::cast_from(0.34631718211362927518e2_f64) * t2328 * t3162;
    let t8028 = t237 * t3113;
    (t8015, t8016, t8017, t8019, t8020, t8021, t8023, t8025, t8027, t8028)
}
