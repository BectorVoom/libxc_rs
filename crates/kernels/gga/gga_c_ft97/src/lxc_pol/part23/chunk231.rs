//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 231/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk231<F: Float>(t1476: F, t852: F, t1486: F, t193: F, t799: F, t27: F, t89: F) -> (F, F, F, F, F) {
    let t1487 = t852 * t1476;
    let t1489 = t1486 * t193 * t1487;
    let t1491 = t799 * t1476;
    let t1493 = t89 * t27 * t1491;
    let t1495 = -t1489 / 6.0 - t1493 / 3.0;
    (t1487, t1489, t1491, t1493, t1495)
}
