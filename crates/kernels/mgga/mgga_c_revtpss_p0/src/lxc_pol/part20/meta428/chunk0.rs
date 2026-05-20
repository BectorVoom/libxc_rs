//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1607/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1607<F: Float>(t1032: F, t1246: F, t12690: F, t12904: F, t3708: F, t11262: F, t1247: F, t3590: F, t3610: F, t3612: F, t13069: F, t3704: F) -> (F, F, F, F, F) {
    let t44267 = t12690 * t1032 * t1246;
    let t44270 = t3708 * t12904;
    let t44273 = t1247 * t11262 * t3590;
    let t44276 = t3610 * t11262 * t3612;
    let t44278 = t13069 * t3704;
    (t44267, t44270, t44273, t44276, t44278)
}
