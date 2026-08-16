//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 786/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk786(t1091: f64, t18685: f64, t10079: f64, t21362: f64, t265: f64, t724: f64, t21355: f64, t2594: f64, t13872: f64, t18188: f64, t18190: f64, t18427: f64, t1901: f64, t21474: f64, t21479: f64, t21483: f64, t21488: f64, t21492: f64, t21496: f64, t21501: f64, t21505: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t21509 = t18685 * t1091;
    let t21510 = t10079 * t21509;
    let t21515 = t724 * t265 * t21362;
    let t21519 = t2594 * t265 * t21355;
    let t21522 = t18188 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t18190 - 2.0_f64 * t446 * t21474 + 2.0_f64 * t446 * t21479 + 2.0_f64 * t446 * t21483 + t446 * t21488 + t446 * t21492 - 2.0_f64 * t446 * t21496 + 2.0_f64 * t446 * t21501 - 2.0_f64 / 3.0_f64 * t1901 * t21505 - 4.0_f64 / 9.0_f64 * t13872 - 2.0_f64 / 3.0_f64 * t1901 * t21510 - 2.0_f64 / 9.0_f64 * t18427 - 2.0_f64 / 3.0_f64 * t446 * t21515 + 4.0_f64 / 9.0_f64 * t446 * t21519;
    (t21509, t21510, t21515, t21519, t21522)
}
