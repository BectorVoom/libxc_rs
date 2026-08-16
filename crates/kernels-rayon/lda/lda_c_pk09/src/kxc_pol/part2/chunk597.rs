//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 597/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk597(t1625: f64, t4755: f64, t1449: f64, t280: f64, t1445: f64, t308: f64, t567: f64, t94: f64, t332: f64, t309: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4756 = t4755 * t1625;
    let t4758 = t1449 * t280;
    let t4759 = t1445 * t4758;
    let t4762 = t308 * t567;
    let t4763 = t4762 * t94;
    let t4764 = t332 * t4763;
    let t4765 = 9.813265947244027_f64 * t4764;
    let t4766 = t567 * t94;
    let t4767 = t309 * t4766;
    (t4756, t4758, t4759, t4762, t4764, t4765, t4767)
}
