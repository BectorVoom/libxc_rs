//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1247/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1247<F: Float>(t6480: F, t8232: F, t1882: F, t26217: F, t26353: F, t8392: F, t26306: F, t26487: F, t26249: F, t100182: F, t102352: F, t110: F, t11431: F, t11496: F, t11810: F, t1901: F, t22959: F, t23257: F, t23266: F, t23323: F, t23339: F, t3271: F, t446: F, t47443: F, t60243: F, t83: F, t8411: F, t91583: F, t91783: F) -> (F,) {
    let t103195 = t8232 * t6480;
    let t103198 = 2.0 / 9.0 * t1882 * t26217;
    let t103200 = 2.0 / 27.0 * t8392 * t26353;
    let t103211 = 2.0 / 27.0 * t8392 * t26306;
    let t103216 = 2.0 / 9.0 * t1882 * t26487;
    let t103219 = 2.0 / 27.0 * t8392 * t26249;
    let t103231 = -4.0 / 3.0 * t1901 * t11810 * t91583 * t3271 + 4.0 / 27.0 * t103195 + t103198 - t103200 - 2.0 / 9.0 * t1901 * t47443 * t23266 - 4.0 / 9.0 * t1901 * t60243 * t22959 - 2.0 / 9.0 * t1901 * t47443 * t23257 + t103211 - t446 * t83 * t102352 / 3.0 + t103216 + 4.0 / 9.0 * t91783 - t103219 - 2.0 / 9.0 * t1901 * t23323 * t11431 - 2.0 * t446 * t8411 * t110 * t100182 - 4.0 / 3.0 * t1901 * t11810 * t23339 * t11496;
    (t103231,)
}
