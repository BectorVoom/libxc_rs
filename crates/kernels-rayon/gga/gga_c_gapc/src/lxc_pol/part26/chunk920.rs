//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 920/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk920(t1030: f64, t11356: f64, t9262: f64, t144: f64, t8448: f64, t1971: f64, t9272: f64, t1734: f64, t5056: f64, t1743: f64, t5703: f64, t3709: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11357 = t1030 * t11356;
    let t11358 = t11357 * t9262;
    let t11360 = t8448 * t144;
    let t11361 = t1971 * t11360;
    let t11362 = t1030 * t11361;
    let t11363 = t11362 * t9272;
    let t11365 = t1734 * t5056;
    let t11367 = t1743 * t11365 * t5703;
    let t11369 = t3709 * t9262;
    (t11357, t11358, t11361, t11362, t11363, t11365, t11367, t11369)
}
