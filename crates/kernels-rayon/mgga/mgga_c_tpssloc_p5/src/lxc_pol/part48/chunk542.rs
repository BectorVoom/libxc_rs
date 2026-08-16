//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 542/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk542(t6553: f64, t6572: f64, t1880: f64, t1902: f64, t798: f64, t1887: f64, t206: f64, t6546: f64) -> (f64, f64, f64, f64) {
    let t6573 = t6553 * t6572;
    let t6574 = t1880 * t6573;
    let t6576 = t798 * t1902;
    let t6579 = t6546 * t206 * t1887;
    (t6573, t6574, t6576, t6579)
}
