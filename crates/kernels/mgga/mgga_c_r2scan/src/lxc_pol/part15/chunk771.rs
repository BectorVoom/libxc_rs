//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 771/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk771<F: Float>(t1551: F, t1632: F, t551: F, t574: F, t1541: F, t545: F, t548: F, t2080: F, t780: F, t1234: F, t566: F, t110: F, t6189: F) -> (F, F, F, F, F, F, F) {
    let t6445 = t551 * t1632 * t1551;
    let t6446 = t574 * t6445;
    let t6448 = t545 * t1541;
    let t6449 = t6448 * t548;
    let t6455 = t2080 * t780;
    let t6457 = t1632 * t1234;
    let t6458 = t551 * t6457;
    let t6459 = t566 * t6458;
    let t6461 = t6189 * t110;
    (t6446, t6448, t6449, t6455, t6457, t6459, t6461)
}
