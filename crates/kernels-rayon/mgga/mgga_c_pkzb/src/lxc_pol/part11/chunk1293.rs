//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1293/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1293(t18427: f64, t18440: f64, t18443: f64, t18445: f64, t27262: f64, t27292: f64, t27295: f64, t31067: f64, t31088: f64, t31204: f64, t31206: f64, t31208: f64, t31210: f64, t31213: f64, t31216: f64, t31218: f64, t31220: f64, t31222: f64, t31225: f64) -> f64 {
    let t31493 = t18440 - 0.93932222222222222223e0_f64 * t18427 + t18443 - 0.905775e0_f64 * t27262 + 0.82785e0_f64 * t27292 + 0.12077e1_f64 * t27295 - 0.485484375e1_f64 * t31204 + 0.58258125e1_f64 * t31206 - 0.3883875e1_f64 * t31208 - 0.3883875e1_f64 * t31210 - 0.1294625e1_f64 * t31213 + 0.6189328125e-1_f64 * t31216 - 0.1237865625e0_f64 * t31218 + 0.247573125e0_f64 * t31220 + 0.247573125e0_f64 * t31222 + 0.82524375e-1_f64 * t31225 - 0.301925e0_f64 * t31067 + 0.905775e0_f64 * t31088 - 0.73586666666666666666e0_f64 * t18445;
    t31493
}
