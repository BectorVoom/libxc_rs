//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 888/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk888(t32077: f64, t32107: f64, t532: f64, t8803: f64, t6879: f64, t225: f64, t8789: f64, t31570: f64, t31616: f64, t31624: f64, t1338: f64, t8788: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32108 = t32077 + t32107;
    let t32110 = t532 * t8803;
    let t32111 = t32110 * t6879;
    let t32120 = t8789 * t225;
    let t32127 = 0.16449340668482264365e-1_f64 * t31570;
    let t32130 = 0.76763589786250567037e-1_f64 * t31616;
    let t32132 = 0.16449340668482264365e-1_f64 * t31624;
    let t32136 = t1338 * t8788;
    (t32108, t32110, t32111, t32120, t32127, t32130, t32132, t32136)
}
