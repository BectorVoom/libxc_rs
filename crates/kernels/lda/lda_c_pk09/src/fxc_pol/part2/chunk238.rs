//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 238/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk238<F: Float>(t666: F, t670: F, t612: F, t616: F, t626: F, t636: F, t653: F, t676: F, t681: F, t687: F, t83: F, t89: F, t110: F, t113: F, t133: F, t131: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1014 = 4.431130547644593 * t666;
    let t1015 = 2.9540870317630623 * t670;
    let t1019 = 0.2946275542389858 * t612;
    let t1020 = 0.1964183694926572 * t616;
    let t1024 = t1014 + t1015 + 4.431130547644593 * t676 + 4.431130547644593 * t681 - 4.431130547644593 * t687 + t1019 + t1020 + 0.2946275542389858 * t626 + 0.2946275542389858 * t636 - 0.2946275542389858 * t653;
    let t1025 = t83 * t1024;
    let t1026 = t1025 * t89;
    let t1029 = 4.59690841536205 * t666;
    let t1030 = 3.0646056102413666 * t670;
    let t1034 = 0.3056501876701794 * t612;
    let t1035 = 0.2037667917801196 * t616;
    let t1039 = t1029 + t1030 + 4.59690841536205 * t676 + 4.59690841536205 * t681 - 4.59690841536205 * t687 + t1034 + t1035 + 0.3056501876701794 * t626 + 0.3056501876701794 * t636 - 0.3056501876701794 * t653;
    let t1040 = t110 * t1039;
    let t1041 = t1040 * t89;
    let t1046 = t133 * t113;
    let t1047 = t131 * t1046;
    (t1014, t1015, t1019, t1020, t1024, t1025, t1026, t1029, t1030, t1034, t1035, t1039, t1040, t1041, t1046, t1047)
}
