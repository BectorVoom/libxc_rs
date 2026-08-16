//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 898/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk898<F: Float>(t35071: F, t35115: F, t35179: F, t35220: F, t160: F, t35149: F, t1969: F, t32879: F, t925: F, t1349: F, t1362: F, t149: F, t32714: F, t34980: F, t35007: F, t35012: F, t35016: F, t35022: F, t35028: F, t35034: F, t35038: F, t35188: F, t35197: F, t35207: F, t5772: F, t6580: F, t6584: F, t6618: F, t6622: F, t7309: F, t7315: F, t7346: F) -> (F, F, F, F) {
    let t35222 = t35071 + t35115 + t35179 + t35220;
    let t35229 = t35149 * t160;
    let t35234 = t1969 * t32879 * t925;
    let t35237 = t1349 * t34980 / F::cast_from(3.0_f64) + t6580 * t7346 / F::cast_from(3.0_f64) + t35007 * t1362 / F::cast_from(6.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1349 * t35012 - t1349 * t35016 / F::cast_from(3.0_f64) - t32714 * t6584 / F::cast_from(18.0_f64) + t5772 * t35022 / F::cast_from(9.0_f64) + t7309 * t6618 / F::cast_from(6.0_f64) - t1349 * t35028 / F::cast_from(3.0_f64) + t7309 * t6622 / F::cast_from(6.0_f64) - t149 * t35222 + F::cast_from(4.0_f64) * t35197 - F::cast_from(12.0_f64) * t35207 + F::cast_from(8.0_f64) * t35038 + F::cast_from(8.0_f64) * t35034 - F::cast_from(2.0_f64) * t35188 + F::cast_from(2.0_f64) * t35229 - t6580 * t7315 / F::cast_from(3.0_f64) - t5772 * t35234 / F::cast_from(9.0_f64);
    (t35222, t35229, t35234, t35237)
}
