//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 966/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk966(t5752: f64, t5757: f64, t1464: f64, t15808: f64, t2012: f64, t3734: f64, t7258: f64, t1014: f64, t7105: f64, t7108: f64, t1489: f64, t7257: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22237 = t5752 * t5757;
    let t22238 = t1464 * t22237;
    let t22240 = t15808 * t2012;
    let t22241 = t1464 * t22240;
    let t22243 = t3734 * t7258;
    let t22244 = t1464 * t22243;
    let t22248 = t1014 * t7105;
    let t22250 = t1014 * t7108;
    let t22252 = t7257 * t1489;
    (t22238, t22241, t22244, t22248, t22250, t22252)
}
