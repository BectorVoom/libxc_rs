//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 964/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk964(t1580: f64, t2487: f64, t10025: f64, t10182: f64, t10184: f64, t10187: f64, t10190: f64, t10193: f64, t10199: f64, t10201: f64, t10204: f64, t10206: f64, t10209: f64, t10216: f64, t1629: f64, t2587: f64, t311: f64, t5422: f64, t5786: f64, t5800: f64, t5803: f64, t5806: f64, t5812: f64, t5815: f64, t5817: f64) -> f64 {
    let t10219 = t1580 * t2487;
    let t10222 = 0.14975624337724558_f64 * t10182 + 0.02466859483068398_f64 * t10184 - 0.02466859483068398_f64 * t10187 - 0.14975624337724558_f64 * t5422 + t10190 * t1629 / 6.0_f64 - t10193 * t10025 / 3.0_f64 + t5786 * t2587 / 6.0_f64 + t10199 / 6.0_f64 + t10201 * t1629 / 6.0_f64 + t10204 / 6.0_f64 + t10206 * t10025 / 3.0_f64 - t10209 / 6.0_f64 + t5800 / 6.0_f64 - t5803 / 6.0_f64 - t5806 / 6.0_f64 - t5812 - t5815 / 12.0_f64 + t5817 / 18.0_f64 - t10216 * t311 / 6.0_f64 - t10219 * t311 / 6.0_f64;
    t10222
}
