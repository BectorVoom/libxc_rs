//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 569/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk569(t3194: f64, t3820: f64, t3330: f64, t3332: f64, t3339: f64, t655: f64, t658: f64, t186: f64, t187: f64, t183: f64, t3384: f64, t3388: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3908 = t3820 * t3194;
    let t3917 = 0.9421211958699838_f64 * t3330;
    let t3918 = 1.2561615944933116_f64 * t3332;
    let t3920 = 0.20936026574888528_f64 * t3339;
    let t3928 = t655 * t658;
    let t3930 = t186 * t186;
    let t3932 = 1.0_f64 / t187 / t3930;
    let t3933 = t183 * t3932;
    let t3943 = 24.0_f64 * t3384;
    let t3944 = 24.0_f64 * t3388;
    (t3908, t3917, t3918, t3920, t3928, t3930, t3933, t3943, t3944)
}
