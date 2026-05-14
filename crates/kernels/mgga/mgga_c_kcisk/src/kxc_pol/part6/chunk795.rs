//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 795/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk795<F: Float>(t1801: F, t28389: F, t1800: F, t1799: F, t28369: F, t10461: F, t22254: F, t2473: F, t6719: F, t8954: F, t6974: F, t8677: F, t1869: F, t15936: F, t8780: F, t2509: F, t8858: F) -> (F, F, F, F, F, F, F) {
    let t28756 = t1801 * t28389;
    let t28757 = t1800 * t28756;
    let t28758 = t1799 * t28757;
    let t28760 = t1801 * t28369;
    let t28761 = t1800 * t28760;
    let t28762 = t10461 * t28761;
    let t28764 = t22254 * t2473;
    let t28765 = t1799 * t28764;
    let t28767 = t6719 * t8954;
    let t28768 = t1799 * t28767;
    let t28775 = t6974 * t8677;
    let t28776 = t1869 * t28775;
    let t28778 = t15936 * t8780;
    let t28779 = t1800 * t28778;
    let t28780 = t1869 * t28779;
    let t28782 = t2509 * t8858;
    (t28758, t28762, t28765, t28768, t28776, t28780, t28782)
}
