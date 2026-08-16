//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1136/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1136(t30921: f64, t35071: f64, t35073: f64, t35075: f64, t35089: f64, t35090: f64, t35093: f64, t35097: f64, t35101: f64, t37366: f64, t37375: f64, t39686: f64, t39690: f64, t39693: f64, t39696: f64, t39700: f64, t39705: f64, t39709: f64) -> f64 {
    let t39711 = -t37366 + 0.42874018118069736972e-3_f64 * t39686 + 0.33020496904084359671e-1_f64 * t39690 + 0.183375e0_f64 * t39693 + 0.13753125e0_f64 * t39696 - t35071 - t35073 - t35075 - t30921 - t37375 - 0.38203125e-2_f64 * t39700 + t35089 + 0.56606566121287473723e-2_f64 * t35090 - t35093 - t35097 - t35101 - 0.21437009059034868486e-2_f64 * t39705 + 0.28303283060643736861e-1_f64 * t39709;
    t39711
}
