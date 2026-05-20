//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 770/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk770<F: Float>(t6141: F, t935: F, t915: F, t2926: F, t6109: F, t2924: F, t2930: F, t4571: F, t6094: F, t6098: F, t6102: F, t1621: F) -> (F, F, F, F, F, F) {
    let t6142 = t6141 * t935;
    let t6144 = F::new(1.0) * t915 * t6142;
    let t6145 = t6109 * t2926;
    let t6147 = F::cast_from(0.16081979498692535067e2_f64) * t2924 * t6145;
    let t6152 = t2930 + F::cast_from(0.11415555555555555555e-1_f64) * t4571 - F::cast_from(0.11415555555555555555e-1_f64) * t6094 + F::cast_from(0.34246666666666666666e-1_f64) * t6098 - F::cast_from(0.17123333333333333333e-1_f64) * t6102;
    let t6157 = t1621 * t1621;
    (t6142, t6144, t6145, t6147, t6152, t6157)
}
