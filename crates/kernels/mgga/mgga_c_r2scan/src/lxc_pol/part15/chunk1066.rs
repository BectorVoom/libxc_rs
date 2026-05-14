//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1066/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1066<F: Float>(t40341: F, t10673: F, t10674: F, t10676: F, t2482: F, t11020: F, t11545: F, t10918: F, t11497: F, t3262: F, t11506: F, t37342: F, t37431: F, t37438: F, t37443: F, t37444: F, t37448: F, t40327: F, t40329: F, t40331: F, t40334: F, t40338: F) -> (F, F, F, F) {
    let t40342 = 0.72042316457491791906e-3 * t40341;
    let t40345 = t10673 * t10674 * t2482 * t10676;
    let t40346 = 0.10248087766267884742e-3 * t40345;
    let t40348 = 5.0 / 16.0 * t11020 * t11545;
    let t40351 = 3.0 / 2.0 * t3262 * t10918 * t11497;
    let t40353 = 3.0 / 4.0 * t11506 * t37342;
    let t40355 = -0.14408463291498358381e-2 * t37431 + 0.20496175532535769484e-3 * t37438 - t40327 - t40329 + 0.81300399444200075504e-3 * t40331 - 0.1951603679568577289e-3 * t40334 + t37443 + t40338 + t40342 - t40346 + t40348 + t40351 - t40353 + 0.60975299583150056628e-3 * t37444 - t37448;
    (t40348, t40351, t40353, t40355)
}
