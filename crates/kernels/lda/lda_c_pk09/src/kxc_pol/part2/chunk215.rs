//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 215/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk215<F: Float>(t666: F, t670: F, t612: F, t616: F, t626: F, t636: F, t653: F, t676: F, t681: F, t687: F, t148: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t793 = F::cast_from(9.625452574583042_f64) * t666;
    let t794 = F::cast_from(6.416968383055361_f64) * t670;
    let t798 = F::new(0.64) * t612;
    let t799 = F::cast_from(0.4266666666666667_f64) * t616;
    let t803 = t793 + t794 + F::cast_from(9.625452574583042_f64) * t676 + F::cast_from(9.625452574583042_f64) * t681 - F::cast_from(9.625452574583042_f64) * t687 + t798 + t799 + F::new(0.64) * t626 + F::new(0.64) * t636 - F::new(0.64) * t653;
    let t804 = F::new(1.0) / t148;
    let t805 = t803 * t804;
    let t806 = t805 * t89;
    (t793, t794, t798, t799, t803, t804, t805, t806)
}
