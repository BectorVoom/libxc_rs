//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1165/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1165<F: Float>(t14092: F, t5176: F, t5175: F, t13260: F, t5077: F, t3337: F, t10707: F, t5091: F, t1797: F, t3365: F, t1816: F, t3354: F) -> (F, F, F, F, F) {
    let t14765 = t5176 * t14092;
    let t14766 = t5175 * t14765;
    let t14768 = t5077 * t13260;
    let t14769 = t3337 * t14768;
    let t14771 = t10707 * t5091;
    let t14773 = t1797 * t3365;
    let t14775 = t3354 * t1816;
    (t14766, t14769, t14771, t14773, t14775)
}
