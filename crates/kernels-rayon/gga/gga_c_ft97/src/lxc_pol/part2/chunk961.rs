//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 961/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk961(t14657: f64, t14683: f64, t14655: f64, t14662: f64, t14666: f64, t14669: f64, t14673: f64, t14676: f64, t14680: f64, t14688: f64, t14692: f64, t14715: f64) -> (f64, f64) {
    let t14929 = 2.0_f64 / 9.0_f64 * t14657;
    let t14936 = 4.0_f64 / 3.0_f64 * t14683;
    let t14939 = 2.0_f64 / 9.0_f64 * t14655 - t14929 + 2.0_f64 / 3.0_f64 * t14662 + t14666 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t14669 - 2.0_f64 / 3.0_f64 * t14673 - 2.0_f64 * t14676 - 4.0_f64 / 3.0_f64 * t14680 - t14936 + 4.0_f64 / 9.0_f64 * t14688 - 4.0_f64 / 3.0_f64 * t14692;
    let t14946 = 4.0_f64 / 27.0_f64 * t14715;
    (t14939, t14946)
}
