//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1131/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1131<F: Float>(t95377: F, t1557: F, t5842: F, t1570: F, t1882: F, t23911: F, t1378: F, t9438: F, t1349: F, t24126: F, t376: F, t23422: F, t5862: F, t8232: F, t5871: F, t23970: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t95378 = 28.0 / 27.0 * t95377;
    let t95379 = t5842 * t1557;
    let t95384 = t5842 * t1570;
    let t95389 = t1882 * t23911;
    let t95403 = t1378 * t9438;
    let t95418 = t1349 * t376 * t24126;
    let t95430 = t1882 * t23422;
    let t95446 = t8232 * t5862;
    let t95448 = t8232 * t5871;
    let t95469 = t1882 * t23970;
    (t95378, t95379, t95384, t95389, t95403, t95418, t95430, t95446, t95448, t95469)
}
