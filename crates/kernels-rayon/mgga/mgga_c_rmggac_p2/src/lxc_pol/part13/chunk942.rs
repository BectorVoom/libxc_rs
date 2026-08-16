//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 942/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk942(t118: f64, t1986: f64, t495: f64, t571: f64, t7717: f64, t2001: f64, t498: f64, t7720: f64, t1618: f64, t1600: f64, t7487: f64, t8352: f64) -> (f64, f64, f64, f64, f64) {
    let t40694 = t1986 * t118 * t571 * t495;
    let t40695 = t7717 * t40694;
    let t40699 = t2001 * t118 * t571 * t498;
    let t40700 = t7720 * t40699;
    let t40702 = t1986 * t1618;
    let t40703 = t7720 * t40702;
    let t40705 = t1986 * t1600;
    let t40706 = t7720 * t40705;
    let t40715 = t7487 * t8352;
    (t40695, t40700, t40703, t40706, t40715)
}
