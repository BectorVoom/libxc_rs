//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 654/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk654(t307: f64, t5759: f64, t1615: f64, t5569: f64, t1435: f64, t1562: f64, t1568: f64, t1633: f64, t280: f64, t1632: f64, t5408: f64, t1614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5760 = t307 * t5759;
    let t5762 = t1615 * t5569;
    let t5773 = t1562 * t1435;
    let t5775 = t1568 * t1435;
    let t5777 = t1633 * t280;
    let t5778 = t1632 * t5777;
    let t5783 = 0.037002892246025966_f64 * t5408;
    let t5785 = t1614 * t280;
    (t5760, t5762, t5773, t5775, t5777, t5778, t5783, t5785)
}
