//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 946/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk946(t1111: f64, t9537: f64, t461: f64, t650: f64, t1114: f64, t242: f64, t3055: f64, t3060: f64, t3052: f64, t3065: f64, t8507: f64, t3124: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9538 = t1111 * t9537;
    let t9540 = t650 * t461;
    let t9542 = t242 * t9540 * t1114;
    let t9543 = t1111 * t9542;
    let t9546 = t242 * t3060 * t3055;
    let t9547 = t3052 * t9546;
    let t9555 = t3065 * t8507;
    let t9556 = t3124 * t9555;
    (t9538, t9540, t9543, t9547, t9555, t9556)
}
