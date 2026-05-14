//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1052/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1052<F: Float>(t687: F, t8747: F, t2388: F, t2379: F, t2385: F, t60: F, t81: F, t9260: F, t684: F, t9261: F, t20: F, t4879: F, t14758: F, t2840: F, t4992: F, t86: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t36951 = t8747 * t687;
    let t36957 = t2388 * t2388;
    let t36958 = 1.0 / t36957;
    let t36962 = t2379 * t2385;
    let t37000 = t60 / t9260 / t81;
    let t37013 = t684 * t9261;
    let t37041 = t4879 * t20;
    let t42385 = t14758 * sigma0;
    let t42530 = t86 * t4992 * t2840;
    (t36951, t36958, t36962, t37000, t37013, t37041, t42385, t42530)
}
