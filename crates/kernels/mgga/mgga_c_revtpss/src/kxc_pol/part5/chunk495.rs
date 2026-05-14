//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 495/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk495<F: Float>(t124: F, t1868: F, t800: F, t1319: F, t1322: F, t1334: F, t1339: F, t1342: F, t1858: F, t1860: F, t225: F, t679: F, t704: F) -> (F, F, F) {
    let t1872 = t124 * t1868;
    let t1873 = t800 * t1872;
    let t1877 = (t679 + t704 - t1319 - t1322 + t1858 + t1334 + t1860 - t1339 - t1342) * t225;
    (t1872, t1873, t1877)
}
