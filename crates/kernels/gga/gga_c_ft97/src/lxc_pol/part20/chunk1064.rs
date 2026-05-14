//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1064/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1064<F: Float>(t108160: F, t108122: F, t108126: F, t108130: F, t108134: F, t108137: F, t108139: F, t108141: F, t108145: F, t108150: F, t108155: F, t108158: F, t27742: F, t713: F, t1434: F, t193: F, t2506: F) -> (F, F, F) {
    let t108161 = t108160 / 6.0;
    let t108162 = 8.0 / 3.0 * t108122 - t108126 - t108130 - t108134 - t108137 - t108139 + t108141 - 2.0 / 3.0 * t108145 - 3.0 * t108150 + t108155 - t108158 - t108161;
    let t108165 = t27742 * t713;
    let t108168 = t1434 * t193 * t2506 * t108165;
    (t108162, t108165, t108168)
}
