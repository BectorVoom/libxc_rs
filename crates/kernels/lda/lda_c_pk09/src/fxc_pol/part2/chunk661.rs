//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 661/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk661<F: Float>(t5212: F, t5068: F, t307: F, t4767: F, t328: F, t319: F, t5039: F, t5161: F, t5045: F, t5190: F, t5208: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5972 = F::cast_from(0.7661514025603425_f64) * t5212;
    let t5974 = F::cast_from(0.033961131963353215_f64) * t5068;
    let t5982 = F::new(2.0) / F::new(27.0) * t307 * t4767;
    let t5984 = F::new(2.0) / F::new(27.0) * t328 * t4767;
    let t5986 = F::new(2.0) / F::new(27.0) * t319 * t4767;
    let t5989 = F::cast_from(0.2946275542389858_f64) * t5039;
    let t5993 = F::cast_from(1.9693913545087083_f64) * t5161;
    let t6002 = F::cast_from(0.1964183694926572_f64) * t5045;
    let t6003 = F::cast_from(0.16411594620905903_f64) * t5190;
    let t6008 = F::cast_from(1.4770435158815312_f64) * t5208;
    let t6009 = F::cast_from(1.4770435158815312_f64) * t5212;
    let t6011 = F::cast_from(0.06547278983088574_f64) * t5068;
    (t5972, t5974, t5982, t5984, t5986, t5989, t5993, t6002, t6003, t6008, t6009, t6011)
}
