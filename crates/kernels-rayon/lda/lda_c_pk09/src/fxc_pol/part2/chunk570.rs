//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 570/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk570(t3393: f64, t3397: f64, t3409: f64, t3332: f64, t3339: f64, t3330: f64, t3424: f64, t3426: f64, t3428: f64, t3444: f64, t3453: f64, t3290: f64, t719: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3945 = 24.0_f64 * t3393;
    let t3946 = 1.3333333333333333_f64 * t3397;
    let t3949 = 6.0_f64 * t3409;
    let t3950 = 0.674354452311972_f64 * t3332;
    let t3951 = 0.112392408718662_f64 * t3339;
    let t3960 = 0.505765839233979_f64 * t3330;
    let t3961 = 16.0_f64 * t3424;
    let t3962 = 16.0_f64 * t3426;
    let t3963 = 16.0_f64 * t3428;
    let t3967 = 6.0_f64 * t3444;
    let t3969 = 16.0_f64 * t3453;
    let t3983 = 2.2140749178833072_f64 * t719 * t3290;
    (t3945, t3946, t3949, t3950, t3951, t3960, t3961, t3962, t3963, t3967, t3969, t3983)
}
