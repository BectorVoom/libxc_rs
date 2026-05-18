//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 597/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk597<F: Float>(t1625: F, t4755: F, t1449: F, t280: F, t1445: F, t308: F, t567: F, t94: F, t332: F, t309: F) -> (F, F, F, F, F, F, F) {
    let t4756 = t4755 * t1625;
    let t4758 = t1449 * t280;
    let t4759 = t1445 * t4758;
    let t4762 = t308 * t567;
    let t4763 = t4762 * t94;
    let t4764 = t332 * t4763;
    let t4765 = F::new(9.813265947244027) * t4764;
    let t4766 = t567 * t94;
    let t4767 = t309 * t4766;
    (t4756, t4758, t4759, t4762, t4764, t4765, t4767)
}
