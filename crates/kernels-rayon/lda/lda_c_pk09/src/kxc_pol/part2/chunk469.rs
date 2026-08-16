//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 469/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk469(t1552: f64, t1554: f64, t1556: f64, t1558: f64, t2502: f64, t2505: f64, t2542: f64, t2546: f64, t300: f64, t306: f64, t2487: f64, t304: f64) -> (f64, f64, f64, f64) {
    let t2566 = t1552 - 1.4770435158815312_f64 * t2542 + t1554 + 1.4770435158815312_f64 * t2546 + t1556 - 0.2946275542389858_f64 * t2502 + t1558 + 0.2946275542389858_f64 * t2505;
    let t2567 = t300 * t2566;
    let t2568 = t2567 * t306;
    let t2571 = t304 * t2487;
    (t2566, t2567, t2568, t2571)
}
