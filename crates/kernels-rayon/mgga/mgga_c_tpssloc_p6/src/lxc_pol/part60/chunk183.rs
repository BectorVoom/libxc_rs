//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 183/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk183(t688: f64, t690: f64, t694: f64, t699: f64, t141: f64, t683: f64) -> (f64, f64, f64, f64) {
    let t701 = -0.632975e0_f64 * t688 - 0.29896666666666666667e0_f64 * t690 - 0.1023875e0_f64 * t694 - 0.82156666666666666667e-1_f64 * t699;
    let t702 = 1.0_f64 / t141;
    let t703 = t701 * t702;
    let t705 = 1.0_f64 * t683 * t703;
    (t701, t702, t703, t705)
}
