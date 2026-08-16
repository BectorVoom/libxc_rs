//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 552/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk552(t341: f64, t4742: f64, t1646: f64, t993: f64, t378: f64, t1647: f64, t1651: f64, t999: f64) -> (f64, f64, f64, f64, f64) {
    let t4743 = t4742 * t341;
    let t4746 = t1646 * t993;
    let t4747 = t4746 * t378;
    let t4752 = t1647 * t378;
    let t4757 = t1651 * t999;
    (t4743, t4746, t4747, t4752, t4757)
}
