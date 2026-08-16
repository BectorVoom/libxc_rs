//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 630/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk630<F: Float>(t1021: F, t5013: F, t1092: F, t1769: F, t2861: F, t1767: F, t2855: F, t1096: F, t1775: F, t1094: F, t1747: F) -> (F, F, F, F, F, F, F, F) {
    let t5014 = t1021 * t5013;
    let t5015 = t1092 * t5014;
    let t5017 = t2861 * t1769;
    let t5019 = t2855 * t1767;
    let t5020 = t1096 * t5019;
    let t5021 = t1092 * t5020;
    let t5023 = t2861 * t1775;
    let t5025 = t1747 * t1094;
    (t5014, t5015, t5017, t5019, t5020, t5021, t5023, t5025)
}
