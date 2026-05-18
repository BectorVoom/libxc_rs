//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 929/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk929<F: Float>(t5092: F, t9890: F, t747: F, t91: F, t3902: F, t3938: F, t18168: F, t18171: F, t18174: F, t10119: F, t14005: F, t18153: F, t18157: F, t18162: F, t18165: F) -> (F, F, F) {
    let t18370 = t9890 * t5092;
    let t18372 = t91 * t18370 * t747;
    let t18375 = t91 * t3902 * t3938;
    let t18381 = t18168 / F::new(9.0);
    let t18382 = F::new(2.0) / F::new(9.0) * t18171;
    let t18383 = F::new(2.0) / F::new(27.0) * t18174;
    let t18384 = F::new(3.0) / F::new(8.0) * t18372 - t18375 / F::new(2.0) + F::new(2.0) * t18153 - t18157 / F::new(3.0) - F::new(6.0) * t18162 + F::new(4.0) * t18165 + t18381 - t18382 + t18383 - t10119 - t14005;
    (t18372, t18375, t18384)
}
