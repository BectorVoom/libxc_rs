//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 639/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk639(t159: f64, t1603: f64, t322: f64, t381: f64, t1639: f64, t377: f64, t550: f64, t980: f64, t1636: f64, t553: f64, t848: f64, t394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4225 = t159 * t1603;
    let t4226 = t4225 * t322;
    let t4228 = 0.13170898365871023197e1_f64 * t381 * t4226;
    let t4230 = 0.13170898365871023197e1_f64 * t377 * t1639;
    let t4231 = t980 * t550;
    let t4234 = 0.13170898365871023197e1_f64 * t377 * t1636;
    let t4235 = t848 * t553;
    let t4237 = t394 * t1603;
    (t4225, t4226, t4228, t4230, t4231, t4234, t4235, t4237)
}
