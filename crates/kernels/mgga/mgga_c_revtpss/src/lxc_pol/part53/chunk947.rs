//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 947/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk947<F: Float>(t34007: F, t572: F, t1916: F, t8614: F, t1518: F, t32374: F, t1568: F, t3140: F, t8477: F, t1497: F, t8621: F, t8622: F, t1469: F, t32591: F, t8442: F, t1493: F, t32600: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34009 = 12.0 * t572 * t34007;
    let t34010 = t1916 * t8614;
    let t34011 = 3.0 * t34010;
    let t34012 = t32374 * t1518;
    let t34013 = t572 * t34012;
    let t34014 = 6.0 * t34013;
    let t34074 = t1568 * t3140;
    let t34075 = t8477 * t34074;
    let t34173 = t8621 * t8622 * t1497;
    let t34176 = t32591 * t1469;
    let t34177 = t8442 * t34176;
    let t34181 = t8621 * t32600 * t1493;
    (t34009, t34011, t34012, t34014, t34074, t34075, t34173, t34177, t34181)
}
