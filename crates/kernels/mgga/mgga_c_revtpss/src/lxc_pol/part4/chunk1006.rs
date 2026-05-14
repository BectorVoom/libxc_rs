//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1006/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1006<F: Float>(t5: F, t13423: F, t117: F, t116: F, t4245: F, t1501: F, t2327: F, t648: F, t670: F, t2371: F, t93: F, t1514: F, t2289: F, t4264: F, t625: F, t4288: F, t10208: F, t1513: F, t2340: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t13424 = piecewise3(t8, 0.0, t13423);
    let t13425 = t13424 * t117;
    let t13426 = t4245 * t116;
    let t13429 = t1501 * t2327;
    let t13435 = t648 * t670;
    let t13440 = t93 * t2371;
    let t13448 = t2289 * t1514;
    let t13451 = 4.0 / 3.0 * t625 * t4264;
    let t13453 = 2.0 / 3.0 * t625 * t4288;
    let t13455 = t10208 * t1513 * t2340;
    (t13425, t13426, t13429, t13435, t13440, t13448, t13451, t13453, t13455)
}
