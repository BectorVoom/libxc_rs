//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 744/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk744<F: Float>(t1060: F, t2185: F, t4668: F, t12362: F, t12571: F, t16679: F, t16745: F, t16748: F, t16751: F, t20536: F, t20540: F, t20551: F, t20666: F, t20669: F, t20779: F, t9166: F) -> (F, F) {
    let t20945 = t2185 * t1060 * t4668;
    let t20961 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t16679 + F::cast_from(6.0_f64) * t20666 - t20669 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t12362 - t9166 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12571 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t20536 - F::cast_from(2.0_f64) * t20540 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t20551 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t20779 + t16745 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t16748 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16751;
    (t20945, t20961)
}
