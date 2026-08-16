//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 401/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk401(t1832: f64, t436: f64, t10: f64, t127: f64, t1655: f64, t1663: f64, t1674: f64, t1676: f64, t1689: f64, t1692: f64, t1695: f64, t1813: f64, t1816: f64, t1819: f64, t1835: f64, t1838: f64, t1840: f64, t1845: f64, t1850: f64, t1852: f64, t411: f64, t426: f64) -> (f64, f64) {
    let t1856 = t436 * t1832;
    let t1859 = -t1655 + t1813 + t1663 + t1816 + t1819 - t1835 + t1674 + t1676 / 6.0_f64 + t1838 / 6.0_f64 + 3.0_f64 / 2.0_f64 * t426 * t10 * t1840 - t426 * t1845 / 2.0_f64 + t1689 + 0.73452_f64 * t1692 + t1695 + 0.73452_f64 * t1850 + 5.87616_f64 * t127 * t1852 * t411 - 1.46904_f64 * t127 * t1856;
    (t1856, t1859)
}
