//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 942/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk942(t13722: f64, t13732: f64, t14317: f64, t14318: f64, t17768: f64, t17773: f64, t17778: f64, t17782: f64, t17787: f64, t17792: f64, t17796: f64, t13739: f64, t13747: f64, t13754: f64, t13781: f64, t13795: f64, t13810: f64, t18142: f64, t18145: f64, t18148: f64, t18363: f64, t18367: f64) -> (f64, f64) {
    let t18567 = 4.0_f64 / 9.0_f64 * t17768 + t17773 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t17778 - 8.0_f64 / 9.0_f64 * t17782 - t14317 - 4.0_f64 / 9.0_f64 * t17787 - 4.0_f64 / 9.0_f64 * t17792 + 4.0_f64 / 27.0_f64 * t17796 - t14318 - 8.0_f64 / 81.0_f64 * t13722 - 4.0_f64 / 27.0_f64 * t13732;
    let t18575 = -8.0_f64 / 27.0_f64 * t13739 - t13747 + t13754 - t18142 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t18145 + t18148 / 9.0_f64 - t13781 + t13795 - t13810 + t18363 / 6.0_f64 - t18367 / 12.0_f64;
    (t18567, t18575)
}
