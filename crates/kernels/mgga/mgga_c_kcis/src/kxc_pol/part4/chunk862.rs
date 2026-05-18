//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 862/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk862<F: Float>(t1371: F, t167: F, t5713: F, t1939: F, t25: F, t493: F, t1938: F, t531: F, t833: F, t3984: F, t3999: F, t1380: F) -> (F, F, F, F, F, F, F, F) {
    let t5714 = t1371 * t167;
    let t5715 = t5713 * t5714;
    let t5718 = t25 * t1939;
    let t5719 = t493 * t5718;
    let t5721 = t1938 * t531;
    let t5722 = t5721 * t833;
    let t5723 = t3984 * t5722;
    let t5726 = t3999 * t1938;
    let t5727 = t5726 * t1380;
    (t5714, t5715, t5719, t5721, t5722, t5723, t5726, t5727)
}
