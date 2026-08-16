//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 831/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk831<F: Float>(t446: F, t5411: F, t1300: F, t2132: F, t2002: F, t3734: F, t1464: F, t1988: F, t3255: F, t3752: F, t544: F, t1650: F, t3754: F) -> (F, F, F, F, F, F, F) {
    let t5412 = t446 * t5411;
    let t5414 = t1300 * t2132;
    let t5415 = t446 * t5414;
    let t5417 = t3734 * t2002;
    let t5418 = t1464 * t5417;
    let t5423 = t3255 * t1988;
    let t5425 = t3752 * t544;
    let t5426 = t3754 * t1650;
    (t5412, t5415, t5417, t5418, t5423, t5425, t5426)
}
