//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1254/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1254<F: Float>(t1593: F, t28374: F, t3999: F, t7908: F, t28343: F, t94246: F, t27369: F, t1014: F, t28525: F, t27484: F, t8151: F, t28473: F) -> (F, F, F, F, F, F, F) {
    let t98845 = t7908 * t1593 * t3999 * t28374;
    let t98847 = t94246 * t28343;
    let t98849 = F::cast_from(0.46336805555555555556e-3_f64) * t7908 * t98847;
    let t98854 = F::cast_from(0.61836467013888888889e-4_f64) * t27369 * t98847;
    let t98863 = t1014 * t28525;
    let t98864 = F::cast_from(0.33163888888888888888e-2_f64) * t98863;
    let t98874 = t8151 * t27484;
    let t98887 = t1014 * t28473;
    (t98845, t98849, t98854, t98863, t98864, t98874, t98887)
}
