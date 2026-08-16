//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1083/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1083(t112: f64, t32594: f64, t671: f64, t8913: f64, t113: f64, t1266: f64, t2114: f64, t2165: f64, t30993: f64, t30995: f64, t31034: f64, t31038: f64, t31046: f64, t31050: f64, t31833: f64, t31834: f64, t31835: f64, t31838: f64, t32572: f64, t510: f64, t650: f64, t652: f64, t7264: f64, t7266: f64, t7271: f64, t7408: f64, t8329: f64, t8860: f64) -> (f64, f64, f64) {
    let t32595 = t32594 * t112;
    let t32605 = t8913 * t671;
    let t32608 = -t113 * t32572 - t1266 * t8860 - 2.0_f64 * t2114 * t7408 - 2.0_f64 * t2165 * t7264 - t32595 * t510 - 2.0_f64 * t32605 * t652 - t650 * t8913 - 4.0_f64 * t7266 * t7271 - t30993 - t30995 - t31034 - t31038 + t31046 + t31050 + 2.0_f64 * t31833 + 2.0_f64 * t31834 - 2.0_f64 * t31835 - 4.0_f64 * t31838 - t8329;
    (t32595, t32605, t32608)
}
