//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 928/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk928(t1389: f64, t3964: f64, t9732: f64, t2735: f64, t546: f64, t1353: f64, t1412: f64, t808: f64, t1369: f64, t2699: f64, t1372: f64, t3943: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9735 = 0.81322168495418382223e-4_f64 * t3964 * t9732 * t1389;
    let t9736 = t2735 * t546;
    let t9737 = t1412 * t1353;
    let t9738 = t808 * t9737;
    let t9739 = t9736 * t9738;
    let t9741 = t2699 * t1369;
    let t9742 = t9741 * t1372;
    let t9744 = t794 * t3943;
    (t9735, t9736, t9737, t9738, t9739, t9741, t9742, t9744)
}
