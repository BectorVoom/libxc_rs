//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1039/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1039<F: Float>(t10707: F, t5091: F, t1797: F, t3365: F, t1816: F, t3354: F, t382: F, t14595: F, t3338: F, t3337: F, t1795: F, t3225: F, t3466: F, t3436: F, t5025: F, t3439: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t14771 = t10707 * t5091;
    let t14773 = t1797 * t3365;
    let t14775 = t3354 * t1816;
    let t14776 = t382 * t14775;
    let t14778 = t3338 * t14595;
    let t14779 = t3337 * t14778;
    let t14781 = t1795 * t3225;
    let t14782 = t14781 * sigma0;
    let t14783 = t14782 * t3466;
    let t14785 = t5025 * t3436;
    let t14786 = t14785 * t3439;
    (t14771, t14773, t14776, t14779, t14783, t14786)
}
