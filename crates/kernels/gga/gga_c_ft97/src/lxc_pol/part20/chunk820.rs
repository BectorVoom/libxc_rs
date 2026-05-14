//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 820/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk820<F: Float>(t25153: F, t25135: F, t799: F, t27: F, t89: F, t2409: F, t2665: F, t6318: F, t6317: F, t1485: F, t458: F) -> (F, F, F, F, F, F) {
    let t25154 = 4.0 / 9.0 * t25153;
    let t25155 = t799 * t25135;
    let t25157 = t89 * t27 * t25155;
    let t25159 = t2665 * t6318 * t2409;
    let t25160 = t6317 * t25159;
    let t25162 = t1485 * t458;
    (t25154, t25155, t25157, t25159, t25160, t25162)
}
