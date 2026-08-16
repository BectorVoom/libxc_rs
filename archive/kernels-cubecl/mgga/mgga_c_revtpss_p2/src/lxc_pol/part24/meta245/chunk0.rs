//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1007/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1007<F: Float>(t1558: F, t2811: F, t2482: F, t1531: F, t37: F, t1544: F, t2475: F, t124: F, t136: F, t243: F, t220: F, t10815: F, t1561: F) -> (F, F, F, F, F, F) {
    let t14597 = t2811 * t1558;
    let t14598 = t2482 * t14597;
    let t14613 = t37 * t1531;
    let t14648 = t2475 * t1544;
    let t14671 = t124 * t1558;
    let t14685 = t243 * t136;
    let t14686 = t14685 * t220;
    let t14712 = t10815 * t1561;
    (t14598, t14613, t14648, t14671, t14686, t14712)
}
