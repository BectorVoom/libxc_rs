//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1280/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1280<F: Float>(t18646: F, t5492: F, t18351: F, t5790: F, t31464: F, t5784: F, t18669: F, t7690: F, t60684: F, t60722: F, t1219: F, t5918: F) -> (F, F, F, F, F, F, F) {
    let t62309 = t5492 * t18646;
    let t62342 = t5790 * t18351;
    let t62345 = t31464 * t5784;
    let t62348 = t7690 * t18669;
    let t62375 = F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t60684;
    let t62390 = F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t60722;
    let t62508 = t1219 * t5918;
    (t62309, t62342, t62345, t62348, t62375, t62390, t62508)
}
