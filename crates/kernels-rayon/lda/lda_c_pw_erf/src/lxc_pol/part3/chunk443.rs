//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 443/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk443(t1664: f64, t1697: f64, t127: f64, t1568: f64, t1655: f64, t1661: f64, t1663: f64, t1667: f64, t1670: f64, t1674: f64, t1676: f64, t1679: f64, t1683: f64, t1689: f64, t1692: f64, t1695: f64, t426: f64, t436: f64) -> f64 {
    let t1698 = t1697 * t1664;
    let t1704 = -t1655 + t1661 + t1663 + t1667 - t1670 + t1674 + t1676 / 3.0_f64 + 3.0_f64 / 2.0_f64 * t426 * t1679 - t426 * t1683 / 2.0_f64 + t1689 + 1.46904_f64 * t1692 + t1695 + 5.87616_f64 * t127 * t1698 - 1.46904_f64 * t127 * t436 * t1568;
    t1704
}
