//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1209/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1209<F: Float>(t20191: F, t382: F, t1195: F, t6723: F, t1187: F, t19593: F, t5181: F, t3437: F, t19735: F, t3438: F, t1809: F, t5086: F) -> (F, F, F, F, F) {
    let t20192 = t382 * t20191;
    let t20194 = t1195 * t6723;
    let t20195 = t1187 * t20194;
    let t20197 = t5181 * t19593;
    let t20198 = t3437 * t20197;
    let t20200 = t3438 * t19735;
    let t20201 = t3437 * t20200;
    let t20203 = t1809 * t5086;
    (t20192, t20195, t20198, t20201, t20203)
}
