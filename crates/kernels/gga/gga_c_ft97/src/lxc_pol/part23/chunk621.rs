//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 621/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk621<F: Float>(t1144: F, t8232: F, t1882: F, t3991: F, t2486: F, t754: F, t3899: F, t8392: F, t2372: F, t255: F) -> (F, F, F, F, F) {
    let t13872 = t8232 * t1144;
    let t13875 = 2.0 / 9.0 * t1882 * t3991;
    let t13879 = t2486 * t754;
    let t13884 = 2.0 / 27.0 * t8392 * t3899;
    let t13885 = t2372 * t255;
    (t13872, t13875, t13879, t13884, t13885)
}
