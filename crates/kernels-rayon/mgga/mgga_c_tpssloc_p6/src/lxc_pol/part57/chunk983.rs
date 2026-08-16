//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 983/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk983(t1880: f64, t29055: f64, t6553: f64, t6571: f64, t25224: f64, t33408: f64, t23270: f64, t25038: f64, t31337: f64, t5527: f64, t121634: f64, t1484: f64, t22986: f64) -> (f64, f64, f64, f64) {
    let t127778 = t1880 * t6553 * t6571 * t29055;
    let t127786 = t1880 * t25224 * t33408;
    let t127790 = t25038 * t23270 * t31337 * t5527;
    let t127794 = t22986 * t23270 * t121634 * t1484;
    (t127778, t127786, t127790, t127794)
}
