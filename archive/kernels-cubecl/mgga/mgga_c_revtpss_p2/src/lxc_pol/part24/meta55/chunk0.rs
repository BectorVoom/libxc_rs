//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 362/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk362<F: Float>(t1376: F, t807: F, t547: F, t786: F, t550: F, t814: F, t816: F, t544: F) -> (F, F, F, F, F) {
    let t1378 = F::cast_from(0.71456696863449561619e-5_f64) * t807 * t1376;
    let t1379 = t786 * t547;
    let t1380 = t814 * t550;
    let t1381 = t1380 * t816;
    let t1383 = F::cast_from(0.12705000702321332056e-4_f64) * t1379 * t1381;
    let t1384 = t544 * t544;
    let t1385 = F::cast_from(1.0_f64) / t1384;
    (t1378, t1379, t1383, t1384, t1385)
}
