//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1162/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1162(t4621: f64, t7789: f64, t5314: f64, t5341: f64, t7773: f64, t5329: f64) -> (f64, f64, f64, f64) {
    let t28145 = t7789 * t4621;
    let t28146 = t5314 * t28145;
    let t28152 = t7773 * t5341;
    let t28153 = t5329 * t28152;
    (t28145, t28146, t28152, t28153)
}
