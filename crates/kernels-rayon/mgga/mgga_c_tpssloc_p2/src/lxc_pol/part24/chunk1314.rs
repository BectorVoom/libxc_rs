//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1314/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1314(t23005: f64, t6579: f64, t2631: f64, t852: f64, t1888: f64, t232: f64, t6646: f64, t23181: f64, t2710: f64, t828: f64, t22996: f64, t2632: f64) -> (f64, f64, f64, f64, f64) {
    let t81697 = t6579 * t23005;
    let t81699 = t852 * t2631;
    let t81702 = t1888 * t6646 * t81699 * t232;
    let t81704 = t6579 * t23181;
    let t81709 = t1888 * t6646 * t2710 * t828 * t232;
    let t81713 = t1888 * t22996 * t81699 * t2632;
    (t81697, t81702, t81704, t81709, t81713)
}
