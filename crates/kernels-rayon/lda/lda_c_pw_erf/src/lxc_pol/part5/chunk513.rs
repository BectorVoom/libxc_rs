//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 513/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk513(t2266: f64, t2538: f64, t2542: f64, t2546: f64, t2548: f64, t2552: f64, t2556: f64, t2560: f64, t2564: f64, t2568: f64, t2569: f64, t2570: f64) -> f64 {
    let t2572 = t2538 + t2542 + t2546 + t2548 + t2552 + t2556 - t2560 - t2564 - t2568 + t2569 + t2570 - 4.0_f64 / 45.0_f64 * t2266;
    t2572
}
