//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2445/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2445<F: Float>(t11804: F, t11921: F, t247: F, t4837: F, t1063: F, t11169: F, t3109: F, t1011: F, t11758: F, t140: F, t11823: F, t11828: F) -> (F, F, F, F, F) {
    let t42487 = t4837 * t247 * t11921 * t11804;
    let t42496 = t1063 * t247 * t3109 * t11169;
    let t42499 = t1011 * t140 * t11758;
    let t42506 = t1011 * t140 * t11823;
    let t42516 = t1011 * t140 * t11828;
    (t42487, t42496, t42499, t42506, t42516)
}
