//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1042/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1042<F: Float>(t95368: F, t1374: F, t2999: F, t89: F, t1557: F, t5842: F, t1570: F, t1378: F, t9438: F, t5862: F, t8232: F, t5871: F, t2178: F, t5968: F, t358: F, t5958: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t95369 = 14.0 / 27.0 * t95368;
    let t95377 = t89 * t2999 * t1374;
    let t95378 = 28.0 / 27.0 * t95377;
    let t95379 = t5842 * t1557;
    let t95384 = t5842 * t1570;
    let t95403 = t1378 * t9438;
    let t95446 = t8232 * t5862;
    let t95448 = t8232 * t5871;
    let t95521 = t2178 * t5968;
    let t95541 = t5968 * t358;
    let t95632 = t8232 * t5958;
    (t95369, t95377, t95378, t95379, t95384, t95403, t95446, t95448, t95521, t95541, t95632)
}
