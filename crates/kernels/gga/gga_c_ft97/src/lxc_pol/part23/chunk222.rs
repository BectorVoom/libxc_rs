//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 222/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk222<F: Float>(t1403: F, t1427: F, t1450: F, t1454: F, t1456: F, t247: F, t2: F, t788: F, t4: F) -> (F, F, F) {
    let t1459 = t1403 * t1427 / 6.0 - t247 * t1454 + 2.0 * t1456 - 2.0 * t1450;
    let t1464 = t788 * t2;
    let t1465 = t1464 * t4;
    (t1459, t1464, t1465)
}
