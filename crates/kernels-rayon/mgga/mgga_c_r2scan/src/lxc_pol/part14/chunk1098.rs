//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1098/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1098(t11002: f64, t38770: f64, t11336: f64, t2262: f64, t3270: f64, t1120: f64, t6692: f64, t37038: f64, t37075: f64, t11217: f64, t833: f64, t1299: f64, t3506: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38771 = t11002 * t38770;
    let t38775 = t3270 * t11336 * t2262;
    let t38783 = t1120 * t6692;
    let t38792 = 308.0_f64 / 27.0_f64 * t37038;
    let t38808 = 308.0_f64 / 27.0_f64 * t37075;
    let t38834 = t11217 * t833;
    let t38839 = t3506 * t1299;
    (t38771, t38775, t38783, t38792, t38808, t38834, t38839)
}
