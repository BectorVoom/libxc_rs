//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 955/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk955<F: Float>(t1882: F, t33676: F, t33751: F, t33784: F, t7543: F, t8232: F, t33761: F, t8392: F, t33721: F, t33632: F, t33642: F, t33687: F) -> (F, F, F, F, F, F, F, F, F) {
    let t142137 = t1882 * t33676;
    let t142146 = t1882 * t33751;
    let t142190 = t1882 * t33784;
    let t142193 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8232 * t7543;
    let t142207 = t8392 * t33761;
    let t142213 = t1882 * t33721;
    let t142219 = t1882 * t33632;
    let t142224 = t1882 * t33642;
    let t142234 = t1882 * t33687;
    (t142137, t142146, t142190, t142193, t142207, t142213, t142219, t142224, t142234)
}
