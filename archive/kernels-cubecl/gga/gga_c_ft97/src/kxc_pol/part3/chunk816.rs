//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 816/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk816<F: Float>(t16686: F, t363: F, t9073: F, t446: F, t15756: F, t569: F, t3281: F, t4462: F, t558: F, t1969: F, t15768: F, t12307: F, t12309: F, t12311: F, t12328: F, t12357: F, t12359: F, t12366: F, t12913: F, t16668: F, t16673: F, t16677: F, t16679: F, t16684: F, t8796: F, t9065: F, t9072: F) -> (F, F, F, F, F, F, F) {
    let t16687 = t16686 * t363;
    let t16688 = t9073 * t16687;
    let t16689 = t446 * t16688;
    let t16691 = t569 * t15756;
    let t16692 = t3281 * t16691;
    let t16694 = t4462 * t558;
    let t16695 = t1969 * t16694;
    let t16696 = t446 * t16695;
    let t16698 = t569 * t15768;
    let t16699 = t446 * t16698;
    let t16704 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16668 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16673 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t16677 - t16679 / F::cast_from(27.0_f64) + t16684 / F::cast_from(18.0_f64) - t16689 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t16692 + t16696 / F::cast_from(18.0_f64) + t16699 / F::cast_from(9.0_f64) - t12307 - t12309 + t12311 - t12328 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9065 - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t8796 - t12357 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t12359 - t12913 - t9072 + t12366;
    (t16687, t16689, t16692, t16694, t16696, t16699, t16704)
}
