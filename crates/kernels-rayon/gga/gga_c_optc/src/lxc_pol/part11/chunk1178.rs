//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1178/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1178(t15204: f64, t1570: f64, t17536: f64, t17539: f64, t17542: f64, t23: f64, t429: f64, t17500: f64, t3058: f64, t17650: f64, t4215: f64, t17618: f64, t4296: f64) -> (f64, f64, f64, f64, f64) {
    let t53327 = t1570 * t15204;
    let t53332 = t17536 * t17539 * t23 * t429 * t17542;
    let t53361 = t3058 * t17500;
    let t53367 = t17650 * t4215;
    let t53390 = t17618 * t4296;
    (t53327, t53332, t53361, t53367, t53390)
}
