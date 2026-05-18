//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1304/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1304<F: Float>(t2118: F, t8240: F, t34848: F, t571: F, t34829: F, t575: F, t124429: F, t124435: F, t124438: F, t131133: F, t131134: F, t131135: F, t131148: F, t131155: F, t131159: F, t131170: F, t1456: F, t1458: F, t1464: F, t1921: F, t33317: F, t34830: F, t5808: F, t7542: F, t7560: F, t8241: F, t8249: F, t8901: F) -> F {
    let t131175 = t8240 * t2118;
    let t131177 = t571 * t34848;
    let t131178 = t34829 * t575;
    let t131182 = t1456 * t34848 + t124429 + t131133 + t131134 + t131135 + t1458 * (t131148 + t131155 + t131159 + t131170) + t34830 * t1464 + t124438 + t131175 + t8241 * t7560 + t124435 + t131177 + t131178 + t33317 * t1921 + t7542 * t8249 + t8901 * t5808;
    t131182
}
