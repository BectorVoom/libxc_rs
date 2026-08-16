//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 194/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk194<F: Float>(t746: F, t747: F, t741: F, t737: F, t724: F, t571: F) -> (F, F, F, F, F, F) {
    let t748 = t746 * t747;
    let t749 = t741 * t748;
    let t751 = F::cast_from(1.0_f64) + t737 / F::cast_from(16.0_f64) - t749 / F::cast_from(256.0_f64);
    let t752 = F::cast_from(1.0_f64) / t751;
    let t753 = t724 * t752;
    let t755 = F::cast_from(1.0_f64) + F::cast_from(0.5137e-1_f64) * t571;
    (t748, t749, t751, t752, t753, t755)
}
