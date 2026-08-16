//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1152/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1152(t1544: f64, t18268: f64, t18850: f64, t198: f64, t23106: f64, t23110: f64, t23111: f64, t23114: f64, t23123: f64, t23124: f64, t23127: f64, t23128: f64, t23129: f64, t23130: f64, t23148: f64, t2403: f64, t262: f64, t4541: f64, t765: f64, t9394: f64) -> f64 {
    let t23152 = -9.0_f64 * t1544 * t18268 * t2403 + 9.0_f64 * t1544 * t18850 * t2403 + 6.0_f64 * t198 * t23114 * t262 + 3.0_f64 * t198 * t23148 * t765 + 18.0_f64 * t23111 * t4541 + 18.0_f64 * t23124 * t4541 - t23106 + t23110 + t23123 + t23127 + t23128 + t23129 + t23130 + t9394;
    t23152
}
