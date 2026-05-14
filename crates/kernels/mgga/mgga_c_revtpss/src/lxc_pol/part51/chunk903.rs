//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 903/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk903<F: Float>(t34010: F, t1518: F, t32374: F, t572: F, t1918: F, t33992: F, t33996: F, t33998: F, t34000: F, t34003: F, t34006: F, t34009: F, t573: F, t8607: F, t8616: F, t1568: F, t3140: F) -> (F, F, F, F, F) {
    let t34011 = 3.0 * t34010;
    let t34012 = t32374 * t1518;
    let t34013 = t572 * t34012;
    let t34014 = 6.0 * t34013;
    let t34015 = 3.0 * t1918 * t8607 + t33992 * t573 + 6.0 * t33996 + 12.0 * t33998 + 6.0 * t34000 + t34003 + t34006 + t34009 + t34011 + t34014 + t8616;
    let t34074 = t1568 * t3140;
    (t34011, t34012, t34014, t34015, t34074)
}
