//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1067/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1067<F: Float>(t1518: F, t8453: F, t572: F, t7330: F, t7741: F, t1916: F, t8614: F, t32374: F, t1568: F, t3140: F, t8477: F, t1497: F, t8621: F, t8622: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t34004 = t1518 * t8453;
    let t34006 = F::cast_from(6.0_f64) * t572 * t34004;
    let t34007 = t7330 * t7741;
    let t34009 = F::cast_from(12.0_f64) * t572 * t34007;
    let t34010 = t1916 * t8614;
    let t34011 = F::cast_from(3.0_f64) * t34010;
    let t34012 = t32374 * t1518;
    let t34013 = t572 * t34012;
    let t34014 = F::cast_from(6.0_f64) * t34013;
    let t34074 = t1568 * t3140;
    let t34075 = t8477 * t34074;
    let t34173 = t8621 * t8622 * t1497;
    (t34004, t34006, t34007, t34009, t34011, t34012, t34014, t34074, t34075, t34173)
}
