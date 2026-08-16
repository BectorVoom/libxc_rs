//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 983/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk983(t1652: f64, t1833: f64, t933: f64, t1870: f64, t1872: f64, t436: f64, t473: f64, t1814: f64, t1953: f64, t3338: f64, t770: f64, t1710: f64, t1859: f64) -> (f64, f64, f64, f64, f64) {
    let t14691 = t1652 * t1833 * t933;
    let t14692 = 0.9743416666666667_f64 * t14691;
    let t14698 = t1870 * t473 * t436 * t1872;
    let t14718 = t1814 * t1953;
    let t14724 = t770 * t3338;
    let t14729 = t1859 * t1710;
    (t14692, t14698, t14718, t14724, t14729)
}
