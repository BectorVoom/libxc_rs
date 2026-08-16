//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 943/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk943<F: Float>(t13812: F, t18153: F, t18157: F, t18162: F, t18165: F, t18168: F, t18171: F, t18174: F, t18372: F, t18375: F, t9972: F, t18557: F, t18567: F, t18575: F) -> F {
    let t18585 = t18372 / F::cast_from(8.0_f64) - t18375 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18153 - t18157 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) * t18162 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t18165 + t18168 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t18171 + F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t18174 - t9972 - t13812;
    let t18587 = t18557 + t18567 + t18575 + t18585;
    t18587
}
