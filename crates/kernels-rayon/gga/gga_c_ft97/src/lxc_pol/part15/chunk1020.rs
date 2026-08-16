//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1020/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1020(t1781: f64, t85451: f64, t1791: f64, t8276: f64, t85469: f64, t1780: f64, t38525: f64, t462: f64, t463: f64, t73497: f64, t73504: f64, t73506: f64, t73508: f64, t73574: f64, t73576: f64, t8275: f64, t86023: f64, t86027: f64, t86031: f64) -> (f64, f64, f64, f64) {
    let t86035 = t1781 * t85451;
    let t86039 = t1791 * t85451;
    let t86043 = t8276 * t85469;
    let t86052 = -4.0_f64 / 3.0_f64 * t73497 + 40.0_f64 / 9.0_f64 * t462 * t8275 * t86023 + 8.0_f64 * t462 * t463 * t86027 - t462 * t463 * t86031 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t462 * t1780 * t86035 + 2.0_f64 * t462 * t463 * t86039 - 8.0_f64 * t462 * t1780 * t86043 + 8.0_f64 / 3.0_f64 * t73504 - 8.0_f64 / 3.0_f64 * t73506 - 8.0_f64 / 9.0_f64 * t73508 + t38525 - 4.0_f64 / 3.0_f64 * t73574 + 8.0_f64 / 9.0_f64 * t73576;
    (t86035, t86039, t86043, t86052)
}
