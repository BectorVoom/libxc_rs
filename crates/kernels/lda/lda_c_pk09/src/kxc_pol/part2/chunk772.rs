//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 772/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk772<F: Float>(t7795: F, t7797: F, t7799: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F, t7838: F, t7842: F, t7846: F, t3317: F, t3319: F, t3335: F, t3342: F, t3384: F, t3388: F, t3393: F, t4299: F, t4302: F, t4303: F, t4304: F, t7851: F, t7855: F) -> (F, F) {
    let t9087 = -3.06460561024137 * t7795 + 3.06460561024137 * t7797 + 3.06460561024137 * t7799 - 0.10188339589005964 * t7801 - 0.15282509383508946 * t7805 - 0.15282509383508946 * t7809 - 0.15282509383508946 * t7811 - 0.15282509383508946 * t7814 - 0.15282509383508946 * t7817 - 0.15282509383508946 * t7834 - 2.2984542076810275 * t7838 + 2.2984542076810275 * t7842 + 2.2984542076810275 * t7846;
    let t9097 = 2.2984542076810275 * t7851 + 2.2984542076810275 * t7855 - 0.15282509383508946 * t3335 - 0.10188339589005964 * t3342 + 4.596908415362055 * t3384 + 4.596908415362055 * t3388 - 4.596908415362055 * t3393 + t4299 + t4302 + t4303 - t4304 + 0.15282509383508946 * t3317 + 0.15282509383508946 * t3319;
    (t9087, t9097)
}
