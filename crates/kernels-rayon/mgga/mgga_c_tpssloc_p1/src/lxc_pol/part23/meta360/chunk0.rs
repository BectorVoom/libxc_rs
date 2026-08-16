//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1158/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1158(t10277: f64, t976: f64, t11046: f64, t42387: f64, t10457: f64, t820: f64, t10969: f64, t121: f64, t10213: f64, t41687: f64, t1043: f64, t204: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42444 = t976 * t10277;
    let t42483 = t11046 * t42387;
    let t42488 = t820 * t10457;
    let t42592 = t121 * t10969;
    let t42624 = t10213 * t41687;
    let t42749 = t204 * t1043;
    (t42444, t42483, t42488, t42592, t42624, t42749)
}
