//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 781/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk781(t2164: f64, t4715: f64, t4685: f64, t7122: f64, t4631: f64, t7110: f64, t108: f64, t1256: f64, t110: f64, t3313: f64, t1975: f64, t4733: f64) -> (f64, f64, f64, f64, f64) {
    let t13392 = t2164 * t4715;
    let t13482 = t7122 * t4685;
    let t13487 = t7110 * t4631;
    let t13502 = t1256 * t108;
    let t13503 = t13502 * t110;
    let t13504 = t3313 * t13503;
    let t13509 = t4733 * t1975;
    (t13392, t13482, t13487, t13504, t13509)
}
