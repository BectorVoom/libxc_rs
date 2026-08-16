//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1164/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1164(t29502: f64, t651: f64, t196: f64, t197: f64, t6773: f64, t2035: f64, t5920: f64, t94: f64, t1937: f64, t7732: f64, t7735: f64, t21663: f64, t38: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29504 = 4.0_f64 * t651 * t29502;
    let t29506 = t6773 * t196 * t197;
    let t29507 = t29506 * t2035;
    let t29508 = t94 * t5920;
    let t29510 = 2.0_f64 * t29508 * t1937;
    let t29512 = 4.0_f64 * t7732 * t7735;
    let t29513 = t21663 * t38;
    (t29504, t29506, t29507, t29508, t29510, t29512, t29513)
}
