//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 998/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk998<F: Float>(t10275: F, t10278: F, t10284: F, t10287: F, t10295: F, t13261: F, t13262: F, t13263: F, t13264: F, t13265: F, t13266: F, t4171: F, t602: F, t1466: F, t2246: F, t1497: F, t2248: F) -> (F, F, F, F) {
    let t13267 = t13261 + t13262 - t10275 - t10278 + t13263 + t13264 - t10284 - t10287 + t13265 + t13266 - t10295;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    let t13283 = t1497 * t2248;
    (t13267, t13269, t13272, t13283)
}
