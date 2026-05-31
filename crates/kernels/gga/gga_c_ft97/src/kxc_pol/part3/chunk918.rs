//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 918/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk918<F: Float>(t2336: F, t4926: F, t89: F, t4918: F, t9725: F, t13723: F, t13732: F, t13740: F, t14327: F, t14329: F, t14336: F, t14341: F, t14346: F, t14347: F, t18142: F, t18145: F, t18148: F, t18153: F, t18157: F, t18162: F, t18165: F, t18168: F, t9699: F) -> (F, F, F) {
    let t18171 = t89 * t2336 * t4926;
    let t18174 = t89 * t9725 * t4918;
    let t18176 = -t13723 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t13732 - t13740 - t14327 + t14329 - t18142 / F::cast_from(6.0_f64) - t18145 / F::cast_from(9.0_f64) + t18148 / F::cast_from(18.0_f64) - t14336 + t14341 - t14346 + t18153 / F::cast_from(3.0_f64) - t18157 / F::cast_from(18.0_f64) - t18162 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18165 + t18168 / F::cast_from(54.0_f64) - t18171 / F::cast_from(27.0_f64) + t18174 / F::cast_from(81.0_f64) - t9699 - t14347;
    (t18171, t18174, t18176)
}
