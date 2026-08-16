//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 399/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk399(t120: f64, t1832: f64, t102: f64, t156: f64, t763: f64, t426: f64, t411: f64, t767: f64, t128: f64, t10: f64, t431: f64, t325: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1833 = t120 * t1832;
    let t1835 = 2.923025_f64 * t102 * t1833;
    let t1837 = t156 * t763;
    let t1838 = t426 * t1837;
    let t1840 = t767 * t411;
    let t1844 = t128 * t1832;
    let t1845 = t10 * t1844;
    let t1849 = t431 * t767;
    let t1850 = t1849 * t325;
    (t1833, t1835, t1837, t1838, t1840, t1844, t1845, t1849, t1850)
}
