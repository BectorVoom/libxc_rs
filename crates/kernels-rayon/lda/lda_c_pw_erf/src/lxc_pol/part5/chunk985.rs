//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 985/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk985(t14795: f64, t2061: f64, t5512: f64, t14639: f64, t1686: f64, t1852: f64, t14650: f64, t5592: f64, t1840: f64, t426: f64, t474: f64, t14584: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14796 = 2.93808_f64 * t14795;
    let t14797 = t5512 * t2061;
    let t14802 = t1686 * t1852 * t14639;
    let t14803 = 5.87616_f64 * t14802;
    let t14813 = t5592 * t14650;
    let t14814 = 11.75232_f64 * t14813;
    let t14816 = t426 * t474 * t1840;
    let t14817 = 2.0_f64 * t14816;
    let t14843 = t426 * t14584;
    (t14796, t14797, t14803, t14814, t14817, t14843)
}
