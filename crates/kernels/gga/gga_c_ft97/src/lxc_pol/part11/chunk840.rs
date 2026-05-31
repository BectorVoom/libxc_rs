//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 840/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk840<F: Float>(t37334: F, t1882: F, t7812: F, t28: F, t7755: F, t8183: F, t89: F, t1581: F, t7773: F, t1554: F, t1636: F, t1560: F) -> (F, F, F, F, F, F, F) {
    let t37335 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t37334;
    let t37336 = t1882 * t7812;
    let t37340 = t89 * t28 * t7755 * t8183;
    let t37343 = t89 * t7773 * t1581;
    let t37344 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t37343;
    let t37345 = t1636 * t1554;
    let t37347 = t89 * t37345 * t1560;
    (t37335, t37336, t37340, t37343, t37344, t37345, t37347)
}
