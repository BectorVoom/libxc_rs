//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 662/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk662(t4567: f64, t5302: f64, t1262: f64, t1662: f64, t3515: f64, t421: f64, t993: f64) -> (f64, f64, f64, f64) {
    let t5303 = t5302 * t4567;
    let t5306 = t1662 * t1262;
    let t5307 = t3515 * t5306;
    let t5310 = t993 * t421;
    (t5303, t5306, t5307, t5310)
}
