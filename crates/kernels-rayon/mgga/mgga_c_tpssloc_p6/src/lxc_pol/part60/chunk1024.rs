//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1024/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1024(t28860: f64, t8607: f64, t19596: f64, t1983: f64, t8640: f64, t1458: f64, t33553: f64, t652: f64, t1873: f64, t29197: f64, t2018: f64, t24432: f64, t24995: f64, t6330: f64) -> (f64, f64, f64, f64, f64) {
    let t128475 = t8607 * t28860;
    let t128477 = t1983 * t8640 * t19596;
    let t128482 = 4.0_f64 * t652 * t33553 * t1458;
    let t128485 = 2.0_f64 * t652 * t29197 * t1873;
    let t128492 = 6.0_f64 * t24995 * t24432 * t2018 * t6330;
    (t128475, t128477, t128482, t128485, t128492)
}
