//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 784/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk784(t12957: f64, t1441: f64, t12939: f64, t1407: f64, t2754: f64, t587: f64, t9438: f64, t9439: f64, t40076: f64, t30829: f64, t31769: f64, t544: f64, t913: f64) -> (f64, f64, f64, f64, f64) {
    let t41698 = t1441 * t12957;
    let t41705 = t1407 * t12939;
    let t41711 = t587 * t9438 * t9439 * t2754;
    let t41736 = 0.25561950635947166451e0_f64 * t40076;
    let t41884 = t544 * t30829 * t913 * t31769;
    (t41698, t41705, t41711, t41736, t41884)
}
