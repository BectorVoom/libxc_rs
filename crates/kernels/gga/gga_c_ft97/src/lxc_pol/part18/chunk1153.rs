//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1153/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1153<F: Float>(t100309: F, t1882: F, t25975: F, t25978: F, t22993: F, t3204: F, t446: F, t7824: F, t1643: F, t6469: F, t38268: F, t26001: F, t379: F, t100293: F, t100295: F, t100298: F, t100302: F, t100305: F, t100308: F) -> (F, F, F, F, F, F, F, F, F) {
    let t100310 = 4.0 / 27.0 * t100309;
    let t100311 = t1882 * t25975;
    let t100312 = 4.0 / 27.0 * t100311;
    let t100313 = t1882 * t25978;
    let t100314 = 4.0 / 81.0 * t100313;
    let t100315 = t22993 * t3204;
    let t100317 = t446 * t7824 * t100315;
    let t100319 = t6469 * t1643;
    let t100321 = t446 * t38268 * t100319;
    let t100323 = t26001 * t379;
    let t100325 = t446 * t7824 * t100323;
    let t100327 = t100293 + t100295 - t100298 / 18.0 + 4.0 * t100302 - 11.0 / 27.0 * t100305 - t100308 + t100310 + t100312 - t100314 - 4.0 / 9.0 * t100317 - 4.0 / 27.0 * t100321 - 4.0 / 9.0 * t100325;
    (t100311, t100313, t100315, t100317, t100319, t100321, t100323, t100325, t100327)
}
