//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2885/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2885<F: Float>(t324: F, t52345: F, t52366: F, t11507: F, t1633: F, t11409: F, t11410: F, t1622: F, t41813: F, t52153: F, t52156: F, t52159: F, t52162: F, t52166: F, t52170: F, t52174: F, t52176: F, t52178: F, t52180: F, t52182: F, t52185: F, t972: F) -> (F, F) {
    let t52368 = (t52345 + t52366) * t324;
    let t52370 = t11507 * t1633;
    let t52377 = t52153 + t52156 + t52159 - t52162 - t52166 - t52170 - t52174 + t52176 + t52178 - t52180 - t52182 + t52185 - F::cast_from(0.19751673498613801407e-1_f64) * t52368 + F::cast_from(0.30762056574649219974e4_f64) * t52370 * t41813 * t972 - F::new(24.0) * t11409 * t1622 * t11410;
    (t52368, t52377)
}
