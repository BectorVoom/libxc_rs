//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 761/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk761(t1887: f64, t206: f64, t22715: f64, t242: f64, t6612: f64, t812: f64, t234: f64, t852: f64, t117: f64, t229: f64, t67: f64, t6559: f64) -> (f64, f64, f64, f64) {
    let t23143 = t22715 * t206 * t1887;
    let t23145 = t6612 * t242;
    let t23146 = t812 * t23145;
    let t23153 = t234 * t852;
    let t23163 = t229 * t67 * t117;
    let t23164 = t6559 * t23163;
    (t23143, t23146, t23153, t23164)
}
