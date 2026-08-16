//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 1004/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk1004(t1218: f64, t1253: f64, t14603: f64, t14608: f64, t14616: f64, t14906: f64, t15548: f64, t2649: f64, t2745: f64, t2892: f64, t317: f64, t4027: f64, t4135: f64, t4309: f64, t830: f64, t880: f64) -> f64 {
    let t15549 = -t1218 * t2892 - t1253 * t2649 - t1253 * t2745 - t14906 * t317 - 2.0_f64 * t4027 * t880 - 2.0_f64 * t4135 * t880 - 2.0_f64 * t4309 * t830 - 12.0_f64 * t14603 + 4.0_f64 * t14608 + 8.0_f64 * t14616 + t15548;
    t15549
}
