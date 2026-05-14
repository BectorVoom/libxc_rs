//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 353/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk353<F: Float>(t546: F, t550: F, t808: F, t807: F, t547: F, t786: F, t814: F, t816: F, t544: F) -> (F, F, F, F, F, F, F) {
    let t1376 = t546 * t808 * t550;
    let t1378 = 0.71456696863449561619e-5 * t807 * t1376;
    let t1379 = t786 * t547;
    let t1380 = t814 * t550;
    let t1381 = t1380 * t816;
    let t1383 = 0.12705000702321332056e-4 * t1379 * t1381;
    let t1384 = t544 * t544;
    let t1385 = 1.0 / t1384;
    (t1376, t1378, t1379, t1381, t1383, t1384, t1385)
}
