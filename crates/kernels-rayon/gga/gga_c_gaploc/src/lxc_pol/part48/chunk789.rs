//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 789/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk789(t12957: f64, t1441: f64, t12939: f64, t1407: f64, t2754: f64, t587: f64, t9438: f64, t9439: f64, t12942: f64, t1429: f64, t549: f64, t30829: f64, t31769: f64, t544: f64, t913: f64) -> (f64, f64, f64, f64, f64) {
    let t41698 = t1441 * t12957;
    let t41705 = t1407 * t12939;
    let t41711 = t587 * t9438 * t9439 * t2754;
    let t41731 = t1429 * t549 * t12942;
    let t41884 = t544 * t30829 * t913 * t31769;
    (t41698, t41705, t41711, t41731, t41884)
}
