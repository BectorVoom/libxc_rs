//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 325/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk325<F: Float>(t1422: F, t1426: F, t350: F, t974: F, t275: F, t176: F, t1000: F, t1325: F, t914: F, t1331: F, t1345: F, t1371: F, t1373: F, t1377: F, t1415: F, t277: F, t364: F, t95: F, t962: F, t995: F, t999: F, sigma0: F) -> (F, F, F, F, F) {
    let t1428 = t1422 * t350 - t974 * t1426;
    let t1429 = t1428 * t275;
    let t1431 = t176 * t1429 * sigma0;
    let t1434 = t1000 * t1325;
    let t1435 = t914 * t1434;
    let t1438 = -t1331 + t1345 + t1371 + t1373 - t1377 + 0.25844881434903430496e-2 * t95 * t277 * t1415 * t962 + t1431 * t364 / 2.0 + t995 + t999 * t1435 / 6.0;
    (t1428, t1431, t1434, t1435, t1438)
}
