//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 982/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk982<F: Float>(t313: F, t3262: F, t1035: F, t1103: F, t1018: F, t347: F, t932: F, t3255: F, t3271: F, t3276: F, t3250: F, t41: F, t85: F) -> (F, F, F, F, F, F) {
    let t10297 = t3262 * t313;
    let t10314 = t1103 * t1035;
    let t10324 = t1018 * t932 * t347;
    let t10333 = t3255 * t3271;
    let t10335 = t3255 * t3276;
    let t10338 = t85 * t3250 * t41;
    (t10297, t10314, t10324, t10333, t10335, t10338)
}
