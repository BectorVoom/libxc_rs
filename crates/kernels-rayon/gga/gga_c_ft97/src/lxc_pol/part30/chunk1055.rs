//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1055/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1055(t263: f64, t35516: f64, t35604: f64, t41402: f64, t24412: f64, t27983: f64, t13927: f64, t33595: f64, t13830: f64, t7553: f64, t10157: f64, t1403: f64, t141410: f64, t141420: f64, t141431: f64, t141435: f64, t193: f64, t2354: f64, t27894: f64, t27943: f64, t33502: f64, t3837: f64, t4003: f64, t6002: f64, t684: f64, t7437: f64, t7441: f64, t7443: f64) -> (f64, f64, f64, f64, f64) {
    let t151066 = t35516 * t263;
    let t151077 = t41402 * t35604;
    let t151079 = t24412 * t27983;
    let t151081 = t13927 * t33595;
    let t151092 = t13830 * t7553;
    let t151094 = -t6002 * t2354 * t151066 * t684 / 18.0_f64 + 2.0_f64 * t6002 * t10157 * t33502 * t3837 + t141410 + t7437 * t27943 / 6.0_f64 - 12.0_f64 * t151077 + 8.0_f64 * t151079 + 4.0_f64 * t151081 - t1403 * t193 * t7441 * t4003 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t141420 + t141431 / 54.0_f64 - t27894 * t7443 / 3.0_f64 - t141435 / 9.0_f64 - 2.0_f64 * t151092;
    (t151077, t151079, t151081, t151092, t151094)
}
