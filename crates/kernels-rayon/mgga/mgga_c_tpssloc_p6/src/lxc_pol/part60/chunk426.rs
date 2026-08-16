//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 426/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk426(t1011: f64, t4639: f64, t1019: f64, t1040: f64, t1611: f64, t1626: f64, t225: f64, t1057: f64, t193: f64, t336: f64, t1654: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4640 = t4639 * t1011;
    let t4641 = t4640 * t1019;
    let t4644 = t1611 * t1040;
    let t4660 = t1626 * t225;
    let t4669 = t4639 * t1057;
    let t4700 = t193 * t336;
    let t4721 = t690 * t1654;
    (t4641, t4644, t4660, t4669, t4700, t4721)
}
