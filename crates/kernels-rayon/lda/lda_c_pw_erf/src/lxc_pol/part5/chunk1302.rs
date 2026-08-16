//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1302/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1302(t20816: f64, t20819: f64, t20826: f64, t20829: f64, t20832: f64, t20835: f64, t20837: f64, t20840: f64, t20844: f64, t20848: f64, t20850: f64, t20852: f64, t20854: f64) -> f64 {
    let t23198 = t20816 - t20819 + t20826 - t20829 + t20832 - t20835 - t20837 - t20840 - t20844 + t20848 - t20850 + t20852 + t20854;
    t23198
}
