//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 801/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk801<F: Float>(t1241: F, t7391: F, t1238: F, t1252: F, t2121: F, t2155: F, t3487: F, t3593: F, t498: F, t7282: F, t7283: F, t7288: F, t7291: F, t7296: F, t7303: F, t7306: F, t7349: F, t7351: F, t7356: F) -> (F, F) {
    let t7392 = t1241 * t7391;
    let t7394 = t7282 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t7288 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t7291 + F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t7296 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t7303 + t7306 * t498 + t7349 * t498 - t7351 * t1252 - t3487 * t2155 - t3593 * t2155 + F::cast_from(2.0_f64) * t1238 * t7356 - t1238 * t7392;
    (t7392, t7394)
}
