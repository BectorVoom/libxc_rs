//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1010/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1010<F: Float>(t33960: F, t33962: F, t30226: F, t30234: F, t30240: F, t30249: F, t30251: F, t30253: F, t32397: F, t32398: F, t32401: F, t32403: F, t32404: F, t33956: F, t33966: F, t33974: F, t33979: F) -> (F,) {
    let t36876 = 0.7640625e-2 * t33960;
    let t36877 = 11.0 / 96.0 * t33962;
    let t36887 = -0.42874018118069736972e-2 * t33956 - t36876 + t36877 - t33966 / 64.0 + 0.34299214494455789578e-2 * t30226 + t32397 + t32398 - 0.17149607247227894789e-2 * t30234 + t32401 + 0.42874018118069736972e-3 * t30240 + t32403 - t32404 - 0.18140473443734395377e0 * t30249 - 0.24009450146119052704e-1 * t30251 + 0.17149607247227894789e-1 * t30253 - 0.34299214494455789578e-2 * t33974 + 0.31448092289604152069e-3 * t33979;
    (t36887,)
}
