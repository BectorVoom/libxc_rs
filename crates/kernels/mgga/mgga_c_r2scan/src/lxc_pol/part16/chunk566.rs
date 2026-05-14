//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 566/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk566<F: Float>(t106: F, t2333: F, t3245: F, t97: F, t1418: F, t1421: F, t1424: F, t1459: F, t1463: F, t1470: F, t1480: F, t1488: F, t1511: F, t1526: F, t1529: F, t1533: F, t2872: F, t3020: F, t3036: F, t3038: F) -> (F,) {
    let t3248 = t97 * t106 * t3245 * t2333;
    let t3249 = -t1418 - t1421 - t1424 - t1511 + t1459 - t1526 - 0.4726e1 * t2872 + t3020 + t1470 - t1480 - t1488 - t3038 - t3036 - t1529 + t1463 - t1533 + t3248;
    (t3249,)
}
