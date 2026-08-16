//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1011/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1011<F: Float>(t34004: F, t572: F, t7330: F, t7741: F, t1916: F, t8614: F, t1518: F, t32374: F, t1918: F, t33992: F, t33996: F, t33998: F, t34000: F, t34003: F, t573: F, t8607: F, t8616: F) -> (F, F, F, F, F) {
    let t34006 = F::cast_from(6.0_f64) * t572 * t34004;
    let t34007 = t7330 * t7741;
    let t34009 = F::cast_from(12.0_f64) * t572 * t34007;
    let t34010 = t1916 * t8614;
    let t34011 = F::cast_from(3.0_f64) * t34010;
    let t34012 = t32374 * t1518;
    let t34013 = t572 * t34012;
    let t34014 = F::cast_from(6.0_f64) * t34013;
    let t34015 = F::cast_from(3.0_f64) * t1918 * t8607 + t33992 * t573 + F::cast_from(6.0_f64) * t33996 + F::cast_from(12.0_f64) * t33998 + F::cast_from(6.0_f64) * t34000 + t34003 + t34006 + t34009 + t34011 + t34014 + t8616;
    (t34007, t34011, t34012, t34014, t34015)
}
