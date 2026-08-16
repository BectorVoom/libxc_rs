//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 850/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk850<F: Float>(t16679: F, t13100: F, t13101: F, t16668: F, t16673: F, t16677: F, t16684: F, t16689: F, t16692: F, t16696: F, t16699: F, t12359: F, t12362: F, t12571: F, t13102: F, t13108: F, t13117: F, t13120: F, t16706: F, t9166: F, t9369: F, t9371: F) -> (F, F) {
    let t17214 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16679;
    let t17220 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t16668 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t16673 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t16677 - t17214 + t16684 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t16689 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t16692 + t16696 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t16699 - t13100 - t13101;
    let t17225 = t13102 - t13108 - t9369 - t9371 - t13117 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t12359 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t12362 - t9166 + t13120 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t12571 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16706;
    (t17220, t17225)
}
