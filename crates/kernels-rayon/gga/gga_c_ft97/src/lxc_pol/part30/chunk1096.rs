//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1096/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1096(t143158: f64, t152669: f64, t33820: f64, t10683: f64, t28496: f64, t6317: f64, t6318: f64, t25162: f64, t35843: f64, t143038: f64, t143058: f64, t152659: f64, t152663: f64, t152667: f64, t152671: f64, t152675: f64, t152680: f64, t152686: f64, t152690: f64, t152694: f64, t152698: f64, t152702: f64) -> (f64, f64, f64, f64) {
    let t152704 = t33820 * t143158 * t152669;
    let t152708 = t6317 * t10683 * t6318 * t28496;
    let t152710 = t25162 * t35843;
    let t152712 = -t143038 / 54.0_f64 - t152659 / 3.0_f64 - t152663 / 3.0_f64 - t152667 / 36.0_f64 - 2.0_f64 / 9.0_f64 * t152671 - 4.0_f64 / 9.0_f64 * t152675 + 2.0_f64 / 27.0_f64 * t152680 + t143058 / 27.0_f64 - 5.0_f64 / 4.0_f64 * t152686 - t152690 / 6.0_f64 - t152694 / 8.0_f64 + 8.0_f64 / 3.0_f64 * t152698 + t152702 + 4.0_f64 / 9.0_f64 * t152704 - 2.0_f64 * t152708 + t152710 / 27.0_f64;
    (t152704, t152708, t152710, t152712)
}
