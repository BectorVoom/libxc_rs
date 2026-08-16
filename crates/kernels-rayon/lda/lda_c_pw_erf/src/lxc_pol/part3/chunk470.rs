//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 470/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk470(t133: f64, t1655: f64, t1663: f64, t1717: f64, t1718: f64, t1813: f64, t1816: f64, t1819: f64, t1835: f64, t1845: f64, t1868: f64, t1870: f64, t1871: f64, t1872: f64) -> f64 {
    let t1878 = -t1655 + t1813 + t1663 + t1816 + t1819 - t1835 + t1717 + 0.5747516666666667_f64 * t1718 + 0.5747516666666667_f64 * t1868 + 5.172765_f64 * t1870 * t1871 * t1872 - 1.724255_f64 * t133 * t1845;
    t1878
}
