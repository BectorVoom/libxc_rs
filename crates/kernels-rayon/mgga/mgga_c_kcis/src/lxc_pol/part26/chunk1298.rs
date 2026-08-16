//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1298/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1298(t1464: f64, t1497: f64, t58599: f64, t7923: f64, t1616: f64, t7429: f64, t1307: f64, t22722: f64, t6159: f64, t1394: f64, t20873: f64, t27387: f64) -> (f64, f64, f64, f64) {
    let t102205 = t1464 * t7923 * t58599 * t1497;
    let t102209 = t1616 * t7429;
    let t102221 = t6159 * t22722 * t1307;
    let t102237 = t1394 * t27387 * t20873;
    (t102205, t102209, t102221, t102237)
}
