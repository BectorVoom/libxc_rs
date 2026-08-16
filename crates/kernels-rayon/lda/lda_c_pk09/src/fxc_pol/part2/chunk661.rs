//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 661/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk661(t5212: f64, t5068: f64, t307: f64, t4767: f64, t328: f64, t319: f64, t5039: f64, t5161: f64, t5045: f64, t5190: f64, t5208: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5972 = 0.7661514025603425_f64 * t5212;
    let t5974 = 0.033961131963353215_f64 * t5068;
    let t5982 = 2.0_f64 / 27.0_f64 * t307 * t4767;
    let t5984 = 2.0_f64 / 27.0_f64 * t328 * t4767;
    let t5986 = 2.0_f64 / 27.0_f64 * t319 * t4767;
    let t5989 = 0.2946275542389858_f64 * t5039;
    let t5993 = 1.9693913545087083_f64 * t5161;
    let t6002 = 0.1964183694926572_f64 * t5045;
    let t6003 = 0.16411594620905903_f64 * t5190;
    let t6008 = 1.4770435158815312_f64 * t5208;
    let t6009 = 1.4770435158815312_f64 * t5212;
    let t6011 = 0.06547278983088574_f64 * t5068;
    (t5972, t5974, t5982, t5984, t5986, t5989, t5993, t6002, t6003, t6008, t6009, t6011)
}
