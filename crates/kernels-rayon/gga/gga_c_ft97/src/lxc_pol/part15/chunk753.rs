//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 753/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk753(t1073: f64, t4454: f64, t8654: f64, t20022: f64, t2258: f64, t8660: f64, t20031: f64, t3613: f64, t12165: f64, t12204: f64, t17552: f64, t17554: f64, t17556: f64, t17573: f64, t17626: f64, t17627: f64, t2265: f64, t631: f64, t8718: f64) -> (f64, f64, f64, f64) {
    let t21068 = t8654 * t4454 * t1073;
    let t21072 = t2258 * t8660 * t20022;
    let t21075 = t3613 * t20031;
    let t21085 = -t2265 * t21068 / 3.0_f64 - t631 * t21072 / 3.0_f64 + t2265 * t21075 / 6.0_f64 + 4.0_f64 / 3.0_f64 * t17573 - t17627 + 3.0_f64 * t17626 + 2.0_f64 / 3.0_f64 * t17552 - t17554 / 3.0_f64 - t17556 / 9.0_f64 + 5.0_f64 / 3.0_f64 * t12204 + t8718 + 5.0_f64 / 9.0_f64 * t12165;
    (t21068, t21072, t21075, t21085)
}
