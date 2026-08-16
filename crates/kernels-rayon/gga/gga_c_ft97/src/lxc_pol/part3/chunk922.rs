//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 922/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk922(t17720: f64, t17724: f64, t17729: f64, t17734: f64, t17738: f64, t17742: f64, t17746: f64, t17751: f64, t17755: f64, t17759: f64, t17763: f64, t13722: f64, t13732: f64, t17768: f64, t17773: f64, t17778: f64, t17782: f64, t17787: f64, t17792: f64, t17796: f64, t9863: f64, t9867: f64) -> (f64, f64) {
    let t18241 = 2.0_f64 / 9.0_f64 * t17720;
    let t18252 = -t18241 + t17724 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t17729 - 2.0_f64 / 9.0_f64 * t17734 - 4.0_f64 / 3.0_f64 * t17738 - 2.0_f64 / 3.0_f64 * t17742 - 2.0_f64 * t17746 - 10.0_f64 / 27.0_f64 * t17751 + 8.0_f64 / 9.0_f64 * t17755 + 2.0_f64 / 3.0_f64 * t17759 + 2.0_f64 / 9.0_f64 * t17763;
    let t18262 = 4.0_f64 / 3.0_f64 * t17768 + t17773 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t17778 - 8.0_f64 / 3.0_f64 * t17782 - t9863 - 4.0_f64 / 3.0_f64 * t17787 - 4.0_f64 / 3.0_f64 * t17792 + 4.0_f64 / 9.0_f64 * t17796 - t9867 - 8.0_f64 / 27.0_f64 * t13722 - 4.0_f64 / 9.0_f64 * t13732;
    (t18252, t18262)
}
