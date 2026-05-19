//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 569/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk569<F: Float>(t3194: F, t3820: F, t3330: F, t3332: F, t3339: F, t655: F, t658: F, t186: F, t187: F, t183: F, t3384: F, t3388: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3908 = t3820 * t3194;
    let t3917 = F::cast_from(0.9421211958699838_f64) * t3330;
    let t3918 = F::cast_from(1.2561615944933116_f64) * t3332;
    let t3920 = F::cast_from(0.20936026574888528_f64) * t3339;
    let t3928 = t655 * t658;
    let t3930 = t186 * t186;
    let t3932 = F::new(1.0) / t187 / t3930;
    let t3933 = t183 * t3932;
    let t3943 = F::new(24.0) * t3384;
    let t3944 = F::new(24.0) * t3388;
    (t3908, t3917, t3918, t3920, t3928, t3930, t3933, t3943, t3944)
}
