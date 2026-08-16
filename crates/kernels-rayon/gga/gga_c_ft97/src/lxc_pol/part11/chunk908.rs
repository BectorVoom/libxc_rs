//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 908/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk908(t38402: f64, t38418: f64, t38435: f64, t38637: f64, t488: f64, t1852: f64, t492: f64, t8355: f64, t1820: f64, t102: f64, t8416: f64, t100: f64) -> (f64, f64, f64, f64) {
    let t38640 = t488 * (t38402 + t38418 + t38435 + t38637);
    let t38645 = t1852 * t492 * t8355;
    let t38647 = t1820 * t1820;
    let t38648 = t1852 * t38647;
    let t38651 = 1.0_f64 / t8416 / t102;
    let t38652 = t100 * t38651;
    (t38640, t38645, t38648, t38652)
}
