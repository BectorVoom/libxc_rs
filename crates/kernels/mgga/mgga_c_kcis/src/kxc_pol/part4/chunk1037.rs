//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1037/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1037<F: Float>(t14590: F, t3438: F, t3437: F, t14706: F, t5077: F, t3337: F, t10692: F, t1801: F, t10745: F, t5073: F, t1805: F, t3425: F, t3474: F, t5043: F, t1804: F, t3361: F) -> (F, F, F, F, F, F, F) {
    let t14739 = t3438 * t14590;
    let t14740 = t3437 * t14739;
    let t14742 = t5077 * t14706;
    let t14743 = t3337 * t14742;
    let t14745 = t10692 * t1801;
    let t14747 = t10745 * t5073;
    let t14749 = t3425 * t1805;
    let t14751 = t3474 * t5043;
    let t14753 = t3361 * t1804;
    (t14740, t14743, t14745, t14747, t14749, t14751, t14753)
}
