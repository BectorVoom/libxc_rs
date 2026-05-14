//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 853/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk853<F: Float>(t1314: F, t8232: F, t1339: F, t1755: F, t452: F, t432: F, t5750: F, t103: F, t23128: F, t82: F, t1901: F, t23137: F, t23141: F, t23145: F, t23148: F, t23152: F, t23155: F, t23158: F, t23161: F, t23164: F, t23169: F, t23173: F, t23176: F, t23179: F, t28: F, t446: F, t89: F) -> (F, F, F, F, F) {
    let t23183 = 4.0 / 27.0 * t8232 * t1314;
    let t23185 = t452 * t1339 * t1755;
    let t23189 = t452 * t5750 * t432;
    let t23193 = t82 * t23128 * t103;
    let t23197 = 2.0 / 3.0 * t446 * t23137 - t446 * t23141 / 9.0 - 2.0 / 27.0 * t446 * t23145 + 2.0 / 27.0 * t23148 + t23152 - 2.0 / 3.0 * t446 * t23155 + 2.0 / 3.0 * t446 * t23158 + 4.0 / 3.0 * t446 * t23161 + 4.0 / 3.0 * t446 * t23164 + t446 * t23169 / 3.0 - 2.0 / 9.0 * t1901 * t23173 - 2.0 / 9.0 * t23176 + 2.0 / 3.0 * t446 * t23179 - t23183 - t446 * t23185 / 3.0 - 2.0 / 3.0 * t446 * t23189 + t89 * t28 * t23193 / 3.0;
    (t23183, t23185, t23189, t23193, t23197)
}
