//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 923/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk923<F: Float>(t120975: F, t1401: F, t1412: F, t814: F, t1372: F, t32265: F, t124: F, t1380: F, t1444: F, t800: F, t32705: F, t239: F, t8583: F, t8589: F, t9990: F, t1405: F, t32272: F) -> (F, F, F, F, F, F, F, F) {
    let t120976 = t120975 * t1401;
    let t120980 = t814 * t1412;
    let t120981 = t120980 * t1372;
    let t120982 = t32265 * t120981;
    let t120986 = t1380 * t800 * t124 * t1444;
    let t120987 = t32705 * t120986;
    let t120991 = t8583 * t8589 * t9990 * t239;
    let t120994 = t32272 * t1405;
    (t120976, t120980, t120981, t120982, t120986, t120987, t120991, t120994)
}
