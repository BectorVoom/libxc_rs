//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 757/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk757<F: Float>(t8092: F, t984: F, t7795: F, t7797: F, t7799: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F, t7838: F, t7842: F, t7846: F, t3317: F, t3319: F, t3335: F, t3342: F, t3384: F, t3388: F, t3393: F, t3629: F, t3632: F, t3633: F, t3634: F, t7851: F, t7855: F) -> (F, F, F) {
    let t8760 = t984 * t8092;
    let t8775 = -12.833936766110723 * t7795 + 12.833936766110723 * t7797 + 12.833936766110723 * t7799 - 0.4266666666666667 * t7801 - 0.64 * t7805 - 0.64 * t7809 - 0.64 * t7811 - 0.64 * t7814 - 0.64 * t7817 - 0.64 * t7834 - 9.625452574583042 * t7838 + 9.625452574583042 * t7842 + 9.625452574583042 * t7846;
    let t8785 = 9.625452574583042 * t7851 + 9.625452574583042 * t7855 - 0.64 * t3335 - 0.4266666666666667 * t3342 + 19.250905149166083 * t3384 + 19.250905149166083 * t3388 - 19.250905149166083 * t3393 + t3629 + t3632 + t3633 - t3634 + 0.64 * t3317 + 0.64 * t3319;
    (t8760, t8775, t8785)
}
