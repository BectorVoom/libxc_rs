//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 743/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk743(t4053: f64, t584: f64, t1449: f64, t2349: f64, t662: f64, t103: f64, t2: f64, t100: f64, t1445: f64, t1447: f64, t4050: f64, t657: f64, t663: f64, t92: f64) -> (f64, f64, f64) {
    let t4054 = t4053 * t584;
    let t4059 = t2349 * t1449;
    let t4060 = t4059 * t662;
    let t4063 = t103 * t2;
    let t4064 = t4063 * t584;
    let t4067 = -25.0_f64 / 9.0_f64 * t657 * t1445 + 10.0_f64 / 9.0_f64 * t92 * t4050 + 5.0_f64 / 3.0_f64 * t92 * t4054 - 25.0_f64 / 9.0_f64 * t1447 * t663 + 10.0_f64 / 9.0_f64 * t100 * t4060 - 5.0_f64 / 3.0_f64 * t100 * t4064;
    (t4060, t4064, t4067)
}
