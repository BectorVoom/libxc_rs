//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 610/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk610(t4295: f64, t731: f64, t1184: f64, t1753: f64, t279: f64, t1752: f64, t1746: f64, t1759: f64, t2953: f64, t739: f64, t34: f64, t939: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4296 = t731 * t4295;
    let t4299 = t1753 * t1184 * t279;
    let t4300 = t1752 * t4299;
    let t4304 = t1759 * t1746;
    let t4305 = 2.0538164420033334_f64 * t4304;
    let t4352 = t2953 * t739;
    let t4355 = t939 * t34;
    (t4296, t4299, t4300, t4305, t4352, t4355)
}
