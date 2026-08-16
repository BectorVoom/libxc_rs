//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 215/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk215(t666: f64, t670: f64, t612: f64, t616: f64, t626: f64, t636: f64, t653: f64, t676: f64, t681: f64, t687: f64, t148: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t793 = 9.625452574583042_f64 * t666;
    let t794 = 6.416968383055361_f64 * t670;
    let t798 = 0.64_f64 * t612;
    let t799 = 0.4266666666666667_f64 * t616;
    let t803 = t793 + t794 + 9.625452574583042_f64 * t676 + 9.625452574583042_f64 * t681 - 9.625452574583042_f64 * t687 + t798 + t799 + 0.64_f64 * t626 + 0.64_f64 * t636 - 0.64_f64 * t653;
    let t804 = 1.0_f64 / t148;
    let t805 = t803 * t804;
    let t806 = t805 * t89;
    (t793, t794, t798, t799, t803, t804, t805, t806)
}
