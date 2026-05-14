//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1408/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1408<F: Float>(t1419: F, t2794: F, t22239: F, t22242: F, t22246: F, t22249: F, t22250: F, t22255: F, t22258: F, t22260: F, t22264: F, t22267: F, t22270: F, t22271: F, t26638: F, t21787: F, t22274: F, t22278: F, t22285: F, t22286: F, t22288: F, t22292: F, t22296: F, t22298: F, t22301: F, t22305: F, t22308: F, t22312: F) -> (F, F) {
    let t26641 = t1419 * t2794;
    let t26642 = 36.0 * t26641;
    let t26645 = t26638 - t22239 + t22242 - t22246 + t22249 + 0.79035972088888888885e-2 * t22250 + t22255 + 0.127022098e-2 * t22258 - t26642 + 0.19263893255070628431e1 * t22260 + t22264 + t22267 + t22270 + 0.8004085801973333333e-2 * t22271;
    let t26652 = 0.16008171603946666666e-1 * t22274 - t22278 + t22285 - 0.60030643514799999999e-2 * t22286 - 0.8555696984824314305e2 * t22288 - t22292 - t22296 - 0.20010214504933333333e-2 * t22298 - 0.60030643514799999999e-2 * t22301 + t22305 - t22308 - t22312 - t21787;
    (t26645, t26652)
}
