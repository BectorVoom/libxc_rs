//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1120/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1120(t27120: f64, t27133: f64, t2205: f64, t3670: f64, t11220: f64, t11223: f64, t11230: f64, t1282: f64, t1291: f64, t26877: f64, t26885: f64, t26951: f64, t27095: f64, t27100: f64, t27105: f64, t3664: f64, t3669: f64, t437: f64, t7812: f64, t7823: f64) -> (f64, f64, f64) {
    let t27134 = t27120 + t27133;
    let t27136 = t2205 * t3670;
    let t27139 = -t11220 * t2205 + 4.0_f64 * t11223 * t7812 - 6.0_f64 * t11230 * t27136 - t1282 * t27134 - 2.0_f64 * t1291 * t27100 + t27095 * t437 + 4.0_f64 * t27105 * t3669 - 2.0_f64 * t3664 * t7823 - t26877 - t26885 + t26951;
    (t27134, t27136, t27139)
}
