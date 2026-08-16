//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1154/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1154(t2749: f64, t36042: f64, t36145: f64, t8392: f64, t143038: f64, t143058: f64, t152659: f64, t152663: f64, t152667: f64, t152671: f64, t152675: f64, t152680: f64, t152686: f64, t152690: f64, t152694: f64, t152698: f64, t152702: f64, t152704: f64, t152708: f64, t152710: f64) -> (f64, f64, f64) {
    let t154083 = t2749 * t36042;
    let t154090 = t8392 * t36145;
    let t154111 = -t143038 / 18.0_f64 - t152659 - t152663 - t152667 / 12.0_f64 - 2.0_f64 / 3.0_f64 * t152671 - 4.0_f64 / 3.0_f64 * t152675 + 2.0_f64 / 9.0_f64 * t152680 + t143058 / 9.0_f64 - 15.0_f64 / 4.0_f64 * t152686 - t152690 / 2.0_f64 - 3.0_f64 / 8.0_f64 * t152694 + 8.0_f64 * t152698 + 3.0_f64 * t152702 + 4.0_f64 / 3.0_f64 * t152704 - 6.0_f64 * t152708 + t152710 / 9.0_f64;
    (t154083, t154090, t154111)
}
