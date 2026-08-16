//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 918/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk918(t592: f64, t6328: f64, t3701: f64, t6463: f64, t11987: f64, t6305: f64, t12000: f64, t6312: f64, t1814: f64, t5333: f64, t1338: f64, t6434: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19593 = t592 * t6328;
    let t19596 = t6463 * t3701;
    let t19606 = t11987 * t6305;
    let t19618 = t12000 * t6312;
    let t19654 = t1814 * t5333;
    let t19657 = t1338 * t6434;
    (t19593, t19596, t19606, t19618, t19654, t19657)
}
