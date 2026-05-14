//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 802/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk802<F: Float>(t13722: F, t13732: F, t17768: F, t17773: F, t17778: F, t17782: F, t17787: F, t17792: F, t17796: F, t9863: F, t9867: F, t18145: F, t18148: F, t505: F, t5165: F, t13683: F) -> (F, F, F, F, F) {
    let t18262 = 4.0 / 3.0 * t17768 + t17773 / 3.0 - 2.0 / 3.0 * t17778 - 8.0 / 3.0 * t17782 - t9863 - 4.0 / 3.0 * t17787 - 4.0 / 3.0 * t17792 + 4.0 / 9.0 * t17796 - t9867 - 8.0 / 27.0 * t13722 - 4.0 / 9.0 * t13732;
    let t18265 = 2.0 / 3.0 * t18145;
    let t18266 = t18148 / 3.0;
    let t18270 = t5165 * t505;
    let t18271 = t13683 * t18270;
    (t18262, t18265, t18266, t18270, t18271)
}
