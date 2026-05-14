//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 202/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk202<F: Float>(t1424: F, t743: F, t1434: F, t193: F, t676: F, t27: F, t89: F) -> (F, F, F, F, F) {
    let t1435 = t743 * t1424;
    let t1437 = t1434 * t193 * t1435;
    let t1439 = t676 * t1424;
    let t1441 = t89 * t27 * t1439;
    let t1443 = -t1437 / 6.0 - t1441 / 3.0;
    (t1435, t1437, t1439, t1441, t1443)
}
