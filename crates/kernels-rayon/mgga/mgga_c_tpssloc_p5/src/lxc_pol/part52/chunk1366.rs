//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1366/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1366(t652: f64, t6534: f64, t8103: f64, t26168: f64, t8690: f64, t119799: f64, t2114: f64, t2165: f64, t24932: f64, t24980: f64, t25958: f64, t25965: f64, t26098: f64, t27863: f64, t27888: f64, t31880: f64, t4077: f64, t6539: f64, t7264: f64, t7266: f64, t7408: f64, t7451: f64, t7472: f64, t7670: f64) -> f64 {
    let t122897 = t652 * t8103 * t6534;
    let t122910 = t8690 * t26168;
    let t122912 = -t2114 * t25958 - t2165 * t26098 - 2.0_f64 * t24932 * t7472 - 2.0_f64 * t24980 * t7266 - 2.0_f64 * t25965 * t7266 - 2.0_f64 * t27863 * t6539 - 2.0_f64 * t27888 * t7472 - 2.0_f64 * t31880 * t4077 - t7264 * t7670 - t7408 * t7451 - 3.0_f64 * t119799 - 2.0_f64 * t122897 + 3.0_f64 * t122910;
    t122912
}
