//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 799/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk799(t14224: f64, t18593: f64, t1901: f64, t21641: f64, t21647: f64, t21652: f64, t21657: f64, t21661: f64, t21665: f64, t21669: f64, t21674: f64, t21678: f64, t21682: f64, t21686: f64, t21689: f64, t21693: f64, t446: f64) -> f64 {
    let t21696 = -t446 * t21641 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t18593 + t1901 * t21647 / 3.0_f64 + t1901 * t21652 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t1901 * t21657 - 2.0_f64 / 9.0_f64 * t1901 * t21661 + t1901 * t21665 / 3.0_f64 + t1901 * t21669 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t21674 + 2.0_f64 * t446 * t21678 - t446 * t21682 - 4.0_f64 / 27.0_f64 * t14224 - t446 * t21686 + 2.0_f64 * t446 * t21689 - t446 * t21693 / 3.0_f64;
    t21696
}
