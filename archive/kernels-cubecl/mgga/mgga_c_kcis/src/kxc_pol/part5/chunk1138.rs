//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1138/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1138<F: Float>(t167: F, t1773: F, t3211: F, t3210: F, t13172: F, t13192: F, t4802: F, t4793: F, t4797: F, t1121: F, t6272: F, t4555: F) -> (F, F, F, F, F, F) {
    let t19148 = t167 * t1773;
    let t19149 = t3211 * t19148;
    let t19150 = t3210 * t19149;
    let t19151 = t13172 * t19150;
    let t19153 = t13192 * t4802;
    let t19155 = t13192 * t4793;
    let t19157 = t13192 * t4797;
    let t19159 = t6272 * t1121;
    let t19160 = t4555 * t19159;
    (t19151, t19153, t19155, t19157, t19159, t19160)
}
