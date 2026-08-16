//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1015/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1015(t10586: f64, t10594: f64, t13682: f64, t13688: f64, t19669: f64, t19672: f64, t19675: f64, t19678: f64, t19681: f64, t19684: f64, t19687: f64, t19691: f64, t19693: f64, t19695: f64, t19699: f64, t19703: f64, t19706: f64, t19711: f64, t19716: f64, t19720: f64, t19723: f64, t19727: f64, t3139: f64, t462: f64, t92: f64) -> f64 {
    let t19729 = -8.0_f64 / 3.0_f64 * t3139 * t19669 + t462 * t19672 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t462 * t19675 - 2.0_f64 / 9.0_f64 * t462 * t19678 - 4.0_f64 / 3.0_f64 * t3139 * t19681 + 2.0_f64 / 9.0_f64 * t462 * t19684 + 4.0_f64 / 3.0_f64 * t462 * t19687 - 4.0_f64 / 27.0_f64 * t10586 - 2.0_f64 / 9.0_f64 * t19691 + t19693 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t19695 + 2.0_f64 * t462 * t19699 + 4.0_f64 * t462 * t19703 - t462 * t19706 / 3.0_f64 - 6.0_f64 * t462 * t19711 - 4.0_f64 / 3.0_f64 * t13688 * t19716 - 4.0_f64 / 3.0_f64 * t13688 * t19720 + 4.0_f64 / 9.0_f64 * t13682 * t19723 - t92 * t19727 - t10594;
    t19729
}
