//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 528/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk528<F: Float>(t3330: F, t3332: F, t3339: F, t655: F, t658: F, t186: F, t187: F, t183: F, t3384: F, t3388: F, t3393: F, t3397: F, t3409: F, t3424: F, t3426: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3917 = 0.9421211958699838 * t3330;
    let t3918 = 1.2561615944933116 * t3332;
    let t3920 = 0.20936026574888528 * t3339;
    let t3928 = t655 * t658;
    let t3930 = t186 * t186;
    let t3932 = 1.0 / t187 / t3930;
    let t3933 = t183 * t3932;
    let t3943 = 24.0 * t3384;
    let t3944 = 24.0 * t3388;
    let t3945 = 24.0 * t3393;
    let t3946 = 1.3333333333333333 * t3397;
    let t3949 = 6.0 * t3409;
    let t3950 = 0.674354452311972 * t3332;
    let t3951 = 0.112392408718662 * t3339;
    let t3960 = 0.505765839233979 * t3330;
    let t3961 = 16.0 * t3424;
    let t3962 = 16.0 * t3426;
    (t3917, t3918, t3920, t3928, t3930, t3933, t3943, t3944, t3945, t3946, t3949, t3950, t3951, t3960, t3961, t3962)
}
