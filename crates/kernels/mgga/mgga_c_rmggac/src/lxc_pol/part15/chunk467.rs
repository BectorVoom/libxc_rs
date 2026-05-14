//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 467/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk467<F: Float>(t1480: F, t1486: F, t1488: F, t1491: F, t1878: F, t1882: F, t1885: F, t206: F, t207: F, t470: F, t473: F, t600: F, t602: F, t6218: F, t6224: F, t6232: F, t6235: F, t6241: F, t6244: F) -> (F,) {
    let t6247 = 6.0 * t1480 * t602 + 60.0 * t1486 * t6232 - 24.0 * t1486 * t6235 - 12.0 * t1486 * t6241 - 24.0 * t1488 * t6224 + 6.0 * t1491 * t600 + 3.0 * t1878 * t473 - 12.0 * t1882 * t470 + 3.0 * t1885 * t470 + 3.0 * t206 * t6244 - t207 * t6218;
    (t6247,)
}
