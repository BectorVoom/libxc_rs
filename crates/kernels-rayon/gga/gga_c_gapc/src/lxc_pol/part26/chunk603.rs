//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 603/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk603(t3651: f64, t3652: f64, t3636: f64, t3641: f64, t3647: f64, t209: f64, t1049: f64, t2964: f64) -> (f64, f64, f64, f64) {
    let t3653 = t3651 * t3652;
    let t3655 = 0.60736713313768998074e-4_f64 * t3636 - 0.43449121406768801912e-4_f64 * t3641 - 0.12653481940368541265e-5_f64 * t3647 + 0.27155700879230501195e-5_f64 * t3653;
    let t3656 = t3655 * t209;
    let t3658 = 2.0_f64 * t2964 * t1049;
    let t3659 = t1049 * t1049;
    (t3655, t3656, t3658, t3659)
}
