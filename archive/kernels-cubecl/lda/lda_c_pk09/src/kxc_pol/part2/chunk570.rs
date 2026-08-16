//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 570/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk570<F: Float>(t3393: F, t3397: F, t3409: F, t3332: F, t3339: F, t3330: F, t3424: F, t3426: F, t3428: F, t3444: F, t3453: F, t3290: F, t719: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3945 = F::cast_from(24.0_f64) * t3393;
    let t3946 = F::cast_from(1.3333333333333333_f64) * t3397;
    let t3949 = F::cast_from(6.0_f64) * t3409;
    let t3950 = F::cast_from(0.674354452311972_f64) * t3332;
    let t3951 = F::cast_from(0.112392408718662_f64) * t3339;
    let t3960 = F::cast_from(0.505765839233979_f64) * t3330;
    let t3961 = F::cast_from(16.0_f64) * t3424;
    let t3962 = F::cast_from(16.0_f64) * t3426;
    let t3963 = F::cast_from(16.0_f64) * t3428;
    let t3967 = F::cast_from(6.0_f64) * t3444;
    let t3969 = F::cast_from(16.0_f64) * t3453;
    let t3983 = F::cast_from(2.2140749178833072_f64) * t719 * t3290;
    (t3945, t3946, t3949, t3950, t3951, t3960, t3961, t3962, t3963, t3967, t3969, t3983)
}
