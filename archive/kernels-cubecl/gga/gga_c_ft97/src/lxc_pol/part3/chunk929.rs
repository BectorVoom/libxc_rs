//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 929/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk929<F: Float>(t5092: F, t9890: F, t747: F, t91: F, t3902: F, t3938: F, t18168: F, t18171: F, t18174: F, t10119: F, t14005: F, t18153: F, t18157: F, t18162: F, t18165: F) -> (F, F, F) {
    let t18370 = t9890 * t5092;
    let t18372 = t91 * t18370 * t747;
    let t18375 = t91 * t3902 * t3938;
    let t18381 = t18168 / F::cast_from(9.0_f64);
    let t18382 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18171;
    let t18383 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t18174;
    let t18384 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t18372 - t18375 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) * t18153 - t18157 / F::cast_from(3.0_f64) - F::cast_from(6.0_f64) * t18162 + F::cast_from(4.0_f64) * t18165 + t18381 - t18382 + t18383 - t10119 - t14005;
    (t18372, t18375, t18384)
}
