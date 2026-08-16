//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 362/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk362(t1684: f64, t1735: f64, t1732: f64, t1738: f64, t1755: f64, t1762: f64, t1766: f64, t1769: f64, t543: f64, t452: f64, t337: f64, t534: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1771 = 1.2466946262544771_f64 * t1684;
    let t1773 = 0.41556487541815906_f64 * t1735;
    let t1775 = t1755 - 6.25_f64 * t1762 + t1766 + 6.25_f64 * t1769 + t1771 - 1.2466946262544771_f64 * t1732 + t1773 + 1.2466946262544771_f64 * t1738;
    let t1776 = 1.0_f64 / t543;
    let t1777 = t1775 * t1776;
    let t1778 = t1777 * t452;
    let t1781 = t534 * t337;
    (t1771, t1773, t1775, t1776, t1777, t1778, t1781)
}
