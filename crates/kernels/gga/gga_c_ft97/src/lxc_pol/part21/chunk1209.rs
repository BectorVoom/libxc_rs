//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1209/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1209<F: Float>(t1882: F, t29967: F, t103542: F, t103550: F, t11468: F, t115229: F, t11552: F, t11587: F, t116342: F, t116532: F, t15772: F, t16047: F, t16110: F, t16115: F, t16246: F, t1901: F, t1909: F, t23339: F, t26171: F, t26240: F, t29727: F, t29806: F, t3238: F, t432: F, t446: F, t452: F, t5644: F, t5710: F, t5717: F, t83: F, t92024: F) -> (F,) {
    let t117848 = t1882 * t29967;
    let t117886 = 2.0 / 3.0 * t446 * t452 * t3238 * t26240 - 2.0 / 9.0 * t117848 + 2.0 * t1901 * t26171 * t23339 * t16115 + 2.0 * t1901 * t26171 * t5717 * t16110 + t103542 + 2.0 / 3.0 * t446 * t452 * t5710 * t16047 - t446 * t452 * t29727 * t432 / 3.0 + t92024 - t103550 + t1901 * t1909 * t5717 * t15772 / 9.0 + 2.0 / 27.0 * t1901 * t11587 * t29806 + 2.0 / 3.0 * t446 * t83 * t115229 - 4.0 / 9.0 * t1901 * t11468 * t116532 + 4.0 / 27.0 * t1901 * t11552 * t116342 + t446 * t452 * t16246 * t5644 / 3.0;
    (t117886,)
}
