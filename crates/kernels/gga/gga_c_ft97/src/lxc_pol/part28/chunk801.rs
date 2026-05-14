//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 801/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk801<F: Float>(t160: F, t35149: F, t1969: F, t32879: F, t925: F, t1349: F, t1362: F, t149: F, t32714: F, t34980: F, t35007: F, t35012: F, t35016: F, t35022: F, t35028: F, t35034: F, t35038: F, t35188: F, t35197: F, t35207: F, t35222: F, t5772: F, t6580: F, t6584: F, t6618: F, t6622: F, t7309: F, t7315: F, t7346: F) -> (F, F, F) {
    let t35229 = t35149 * t160;
    let t35234 = t1969 * t32879 * t925;
    let t35237 = t1349 * t34980 / 3.0 + t6580 * t7346 / 3.0 + t35007 * t1362 / 6.0 - 2.0 / 3.0 * t1349 * t35012 - t1349 * t35016 / 3.0 - t32714 * t6584 / 18.0 + t5772 * t35022 / 9.0 + t7309 * t6618 / 6.0 - t1349 * t35028 / 3.0 + t7309 * t6622 / 6.0 - t149 * t35222 + 4.0 * t35197 - 12.0 * t35207 + 8.0 * t35038 + 8.0 * t35034 - 2.0 * t35188 + 2.0 * t35229 - t6580 * t7315 / 3.0 - t5772 * t35234 / 9.0;
    (t35229, t35234, t35237)
}
