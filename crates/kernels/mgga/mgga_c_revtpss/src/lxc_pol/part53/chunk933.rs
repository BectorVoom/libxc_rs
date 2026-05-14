//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 933/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk933<F: Float>(t1937: F, t33602: F, t6985: F, t7735: F, t1497: F, t8441: F, t7714: F, t8621: F, t1493: F, t84: F, t1936: F, t28030: F, t7741: F, t1501: F, t8453: F, t8692: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33603 = t33602 * t1937;
    let t33605 = t6985 * t7735;
    let t33612 = t8441 * t1497;
    let t33620 = t8441 * t7714;
    let t33621 = t8621 * t33620;
    let t33624 = t84 * t1493;
    let t33633 = t28030 * t1936;
    let t33635 = t33602 * t1936;
    let t33637 = t6985 * t7741;
    let t33639 = t1501 * t8453;
    let t33640 = 2.0 * t33639;
    let t33642 = 4.0 * t8692 * t7741;
    (t33603, t33605, t33612, t33620, t33621, t33624, t33633, t33635, t33637, t33639, t33640, t33642)
}
