//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1125/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1125(t123: f64, t21888: f64, t7297: f64, t9647: f64, t16880: f64, t21504: f64, t29439: f64, t9752: f64, t23292: f64, t2558: f64, t1222: f64, t3130: f64) -> (f64, f64, f64, f64, f64) {
    let t29498 = 0.7690526230142224284e-2_f64 * t9647 * t21888 * t123 * t7297;
    let t29501 = 0.3845263115071112142e-2_f64 * t9647 * t16880 * t21504;
    let t29503 = 0.1281754371690370714e-2_f64 * t29439 * t9752;
    let t29631 = 0.64087718584518535698e-3_f64 * t9647 * t23292 * t2558;
    let t29850 = 0.63233348079280332442e-2_f64 * t1222 * t3130;
    (t29498, t29501, t29503, t29631, t29850)
}
