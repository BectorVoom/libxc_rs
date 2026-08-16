//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 936/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk936<F: Float>(t4635: F, t713: F, t2600: F, t2599: F, t14159: F, t3876: F, t13839: F, t3881: F, t4969: F, t724: F, t773: F, t5147: F) -> (F, F, F, F, F) {
    let t18471 = t4635 * t713;
    let t18472 = t2600 * t18471;
    let t18473 = t2599 * t18472;
    let t18476 = t14159 * t3876;
    let t18479 = t13839 * t3881;
    let t18483 = t724 * t773 * t4969;
    let t18486 = t5147 * t713;
    (t18473, t18476, t18479, t18483, t18486)
}
