//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 448/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk448(t120: f64, t133: f64, t474: f64, t1675: f64, t1655: f64, t1661: f64, t1663: f64, t1667: f64, t1670: f64, t1679: f64, t1683: f64) -> (f64, f64, f64) {
    let t1717 = 0.3831677777777778_f64 * t133 * t474 * t120;
    let t1718 = t133 * t1675;
    let t1724 = -t1655 + t1661 + t1663 + t1667 - t1670 + t1717 + 1.1495033333333333_f64 * t1718 + 5.172765_f64 * t133 * t1679 - 1.724255_f64 * t133 * t1683;
    (t1717, t1718, t1724)
}
