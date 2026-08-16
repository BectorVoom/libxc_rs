//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1137/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1137(t22987: f64, t25038: f64, t25248: f64, t2553: f64, t23005: f64, t6579: f64, t2631: f64, t852: f64, t1888: f64, t232: f64, t6646: f64, t23181: f64) -> (f64, f64, f64, f64, f64) {
    let t81695 = t25038 * t25248 * t22987 * t2553;
    let t81697 = t6579 * t23005;
    let t81699 = t852 * t2631;
    let t81702 = t1888 * t6646 * t81699 * t232;
    let t81704 = t6579 * t23181;
    (t81695, t81697, t81699, t81702, t81704)
}
