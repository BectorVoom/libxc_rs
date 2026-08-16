//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 828/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk828(t1458: f64, t8103: f64, t1459: f64, t1849: f64, t2114: f64, t2167: f64, t27863: f64, t28027: f64, t28029: f64, t28032: f64, t28034: f64, t28036: f64, t28038: f64, t28040: f64, t28042: f64, t28047: f64, t28240: f64, t29486: f64, t29497: f64, t510: f64, t5460: f64, t5494: f64, t574: f64, t6287: f64, t6468: f64, t652: f64, t7266: f64, t8107: f64) -> (f64, f64) {
    let t29501 = t8103 * t1458;
    let t29506 = -4.0_f64 * t1459 * t27863 + 2.0_f64 * t1849 * t8107 - t2114 * t6287 + t2167 * t6468 - t29486 * t510 + t29497 * t574 - 4.0_f64 * t29501 * t652 - 4.0_f64 * t5460 * t7266 - 2.0_f64 * t5494 * t7266 - t28027 - t28029 - t28032 - t28034 - t28036 - t28038 - t28040 - t28042 - t28047 + t28240;
    (t29501, t29506)
}
