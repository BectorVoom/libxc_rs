//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 241/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk241(t666: f64, t670: f64, t612: f64, t616: f64, t626: f64, t636: f64, t653: f64, t676: f64, t681: f64, t687: f64, t83: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1014 = 4.431130547644593_f64 * t666;
    let t1015 = 2.9540870317630623_f64 * t670;
    let t1019 = 0.2946275542389858_f64 * t612;
    let t1020 = 0.1964183694926572_f64 * t616;
    let t1024 = t1014 + t1015 + 4.431130547644593_f64 * t676 + 4.431130547644593_f64 * t681 - 4.431130547644593_f64 * t687 + t1019 + t1020 + 0.2946275542389858_f64 * t626 + 0.2946275542389858_f64 * t636 - 0.2946275542389858_f64 * t653;
    let t1025 = t83 * t1024;
    let t1026 = t1025 * t89;
    (t1014, t1015, t1019, t1020, t1024, t1025, t1026)
}
