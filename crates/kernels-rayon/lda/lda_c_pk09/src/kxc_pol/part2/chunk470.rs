//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 470/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk470(t1571: f64, t1573: f64, t1575: f64, t1577: f64, t2502: f64, t2505: f64, t2542: f64, t2546: f64, t314: f64, t306: f64, t2487: f64, t318: f64) -> (f64, f64, f64, f64) {
    let t2578 = t1571 - 0.7661514025603425_f64 * t2542 + t1573 + 0.7661514025603425_f64 * t2546 + t1575 - 0.15282509383508946_f64 * t2502 + t1577 + 0.15282509383508946_f64 * t2505;
    let t2579 = t314 * t2578;
    let t2580 = t2579 * t306;
    let t2583 = t318 * t2487;
    (t2578, t2579, t2580, t2583)
}
