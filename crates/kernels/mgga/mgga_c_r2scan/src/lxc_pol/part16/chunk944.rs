//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 944/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk944<F: Float>(t10666: F, t113: F, t97: F, t3347: F, t5086: F, t1064: F, t23040: F, t1561: F, t3261: F, t122: F, t874: F, t3438: F, t10978: F, t10979: F, t2317: F, t597: F) -> (F, F, F, F, F, F) {
    let t37282 = t97 * t10666 * t113;
    let t37292 = t5086 * t3347;
    let t37299 = t23040 * t1064;
    let t37327 = t97 * t3261 * t1561;
    let t37355 = t874 * t122;
    let t37356 = t3438 * t37355;
    let t37358 = t10978 * t10979 * t2317 * t37356;
    let t37359 = 0.13010691197123848594e-3 * t37358;
    let t37360 = t597 * t37355;
    (t37282, t37292, t37299, t37327, t37359, t37360)
}
