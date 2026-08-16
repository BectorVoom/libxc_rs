//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2045/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2045<F: Float>(t28993: F, t571: F, t101724: F, t104041: F, t104054: F, t1458: F, t1464: F, t18178: F, t18217: F, t1921: F, t2111: F, t2118: F, t26704: F, t28945: F, t3: F, t4154: F, t4168: F, t575: F, t8114: F, t8130: F, t95182: F, t95184: F, t95186: F, t95190: F) -> F {
    let t104062 = F::cast_from(2.0_f64) * t571 * t28993;
    let t104065 = t8114 * t4168 + F::cast_from(2.0_f64) * t95190 + t95186 + F::cast_from(2.0_f64) * t95182 + t2111 * t18217 + t18178 * t2118 + t1458 * (t101724 + t104054) + t26704 * t1921 + t4154 * t8130 + F::cast_from(2.0_f64) * t28945 * t1464 + t104062 + t3 * t104041 * t575 + t95184;
    t104065
}
