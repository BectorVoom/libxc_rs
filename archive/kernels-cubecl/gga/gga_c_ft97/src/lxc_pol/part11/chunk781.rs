//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 781/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk781<F: Float>(t10575: F, t2681: F, t2739: F, t295: F, t9567: F, t9954: F, t2783: F, t458: F, t8282: F, t849: F, t10556: F, t10559: F, t10560: F, t10563: F, t10566: F, t10568: F, t10572: F, t462: F, t92: F) -> (F, F, F, F) {
    let t10577 = t2681 * t10575 * t2739;
    let t10580 = t9567 * t295;
    let t10581 = t10580 * t9954;
    let t10584 = t458 * t2783;
    let t10586 = t8282 * t849;
    let t10588 = -t462 * t10556 / F::cast_from(3.0_f64) + t10559 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t462 * t10560 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t10563 + t462 * t10566 + t462 * t10568 - F::cast_from(6.0_f64) * t92 * t10572 + F::cast_from(6.0_f64) * t462 * t10577 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t462 * t10581 - F::cast_from(2.0_f64) * t10584 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t10586;
    (t10577, t10580, t10581, t10588)
}
