//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 851/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk851<F: Float>(t8092: F, t984: F, t7795: F, t7797: F, t7799: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F, t7838: F, t7842: F, t7846: F) -> (F, F) {
    let t8760 = t984 * t8092;
    let t8775 = -F::new(12.833936766110723) * t7795 + F::new(12.833936766110723) * t7797 + F::new(12.833936766110723) * t7799 - F::new(0.4266666666666667) * t7801 - F::new(0.64) * t7805 - F::new(0.64) * t7809 - F::new(0.64) * t7811 - F::new(0.64) * t7814 - F::new(0.64) * t7817 - F::new(0.64) * t7834 - F::new(9.625452574583042) * t7838 + F::new(9.625452574583042) * t7842 + F::new(9.625452574583042) * t7846;
    (t8760, t8775)
}
