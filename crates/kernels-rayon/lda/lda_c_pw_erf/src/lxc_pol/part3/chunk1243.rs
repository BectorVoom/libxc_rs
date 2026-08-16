//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1243/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1243(t3338: f64, t770: f64, t1710: f64, t1859: f64, t14581: f64, t426: f64, t1849: f64, t1953: f64, t127: f64, t14549: f64, t14552: f64, t14555: f64, t14558: f64, t14561: f64, t14562: f64, t1568: f64, t1664: f64, t1697: f64, t1832: f64, t1852: f64, t3251: f64, t3296: f64, t411: f64, t5548: f64, t5578: f64, t8884: f64) -> (f64, f64, f64, f64, f64) {
    let t14724 = t770 * t3338;
    let t14729 = t1859 * t1710;
    let t14732 = t426 * t14581;
    let t14734 = t1849 * t1953;
    let t14756 = 17.62848_f64 * t127 * t5578 * t1568 + 5.87616_f64 * t127 * t1852 * t3251 - 88.1424_f64 * t127 * t3296 * t1832 * t1664 + 17.62848_f64 * t127 * t1697 * t5548 * t411 + t14549 + t14552 - t14555 + t14558 + t14561 - t14562 - 3.0_f64 / 2.0_f64 * t8884;
    (t14724, t14729, t14732, t14734, t14756)
}
