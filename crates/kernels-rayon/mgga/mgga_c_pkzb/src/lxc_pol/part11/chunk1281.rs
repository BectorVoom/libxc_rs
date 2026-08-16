//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1281/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1281(t18427: f64, t18445: f64, t18554: f64, t18555: f64, t27262: f64, t27292: f64, t27295: f64, t31067: f64, t31088: f64, t31204: f64, t31206: f64, t31208: f64, t31210: f64, t31213: f64, t31216: f64, t31218: f64, t31220: f64, t31222: f64, t31225: f64) -> f64 {
    let t31230 = t18554 - 0.93011851851851851854e0_f64 * t18427 + t18555 - 0.89690000000000000001e0_f64 * t27262 + 0.82156666666666666665e0_f64 * t27292 + 0.11958666666666666667e1_f64 * t27295 - 0.3560484375e1_f64 * t31204 + 0.427258125e1_f64 * t31206 - 0.28483875e1_f64 * t31208 - 0.28483875e1_f64 * t31210 - 0.9494625e0_f64 * t31213 + 0.1151859375e0_f64 * t31216 - 0.230371875e0_f64 * t31218 + 0.46074375e0_f64 * t31220 + 0.46074375e0_f64 * t31222 + 0.15358125e0_f64 * t31225 - 0.29896666666666666667e0_f64 * t31067 + 0.8969e0_f64 * t31088 - 0.7302814814814814815e0_f64 * t18445;
    t31230
}
