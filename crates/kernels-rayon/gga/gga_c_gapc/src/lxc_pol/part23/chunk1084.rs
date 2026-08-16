//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1084/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1084(t11965: f64, t28353: f64, t871: f64, t2511: f64, t3757: f64, t1026: f64, t2786: f64, t3304: f64, t3772: f64, t9913: f64, t11781: f64, t9485: f64) -> (f64, f64, f64, f64, f64) {
    let t33452 = t871 * t11965 * t28353;
    let t33454 = t3757 * t2511;
    let t33457 = t2786 * t1026 * t3304;
    let t33460 = t3772 * t9913;
    let t33462 = t11781 * t9485;
    (t33452, t33454, t33457, t33460, t33462)
}
