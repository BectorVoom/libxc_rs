//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 851/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk851<F: Float>(t1876: F, t452: F, t5710: F, t1339: F, t1651: F, t447: F, t1643: F, t1866: F, t1882: F, t5657: F, t1328: F, t1637: F, t89: F, t1332: F, t1588: F, t1871: F, t488: F) -> (F, F, F, F, F, F, F) {
    let t23137 = t452 * t5710 * t1876;
    let t23141 = t447 * t1339 * t1651;
    let t23145 = t1866 * t1339 * t1643;
    let t23148 = t1882 * t5657;
    let t23152 = 4.0 / 27.0 * t89 * t1637 * t1328;
    let t23153 = t1332 * t1588;
    let t23155 = t1871 * t488 * t23153;
    (t23137, t23141, t23145, t23148, t23152, t23153, t23155)
}
