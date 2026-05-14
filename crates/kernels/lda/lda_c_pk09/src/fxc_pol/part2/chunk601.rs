//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 601/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk601<F: Float>(t319: F, t4767: F, t5039: F, t5161: F, t5045: F, t5190: F, t5208: F, t5212: F, t5068: F, t1435: F, t1594: F, t1597: F, t1610: F, t747: F, t1609: F, t303: F, t337: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5986 = 2.0 / 27.0 * t319 * t4767;
    let t5989 = 0.2946275542389858 * t5039;
    let t5993 = 1.9693913545087083 * t5161;
    let t6002 = 0.1964183694926572 * t5045;
    let t6003 = 0.16411594620905903 * t5190;
    let t6008 = 1.4770435158815312 * t5208;
    let t6009 = 1.4770435158815312 * t5212;
    let t6011 = 0.06547278983088574 * t5068;
    let t6018 = t1594 * t1435;
    let t6020 = t1597 * t1435;
    let t6022 = t747 * t1610;
    let t6023 = t1609 * t6022;
    let t6025 = t303 * t337;
    (t5986, t5989, t5993, t6002, t6003, t6008, t6009, t6011, t6018, t6020, t6022, t6023, t6025)
}
