//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1244/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1244(t2165: f64, t7801: f64, t1459: f64, t1774: f64, t1849: f64, t32350: f64, t32676: f64, t32679: f64, t33554: f64, t33555: f64, t33556: f64, t34137: f64, t34146: f64, t510: f64, t574: f64, t652: f64, t7042: f64, t7943: f64, t7989: f64, t8329: f64, t8690: f64, t8829: f64, t8840: f64) -> (f64, f64) {
    let t34150 = t2165 * t7801;
    let t34157 = -2.0_f64 * t1459 * t32350 - t1774 * t8829 + t1849 * t8840 - t34137 * t510 + t34146 * t574 - 2.0_f64 * t34150 * t652 - 2.0_f64 * t7042 * t7989 - t7943 * t8690 - t32676 - t32679 - t33554 - t33555 - t33556 - t8329;
    (t34150, t34157)
}
