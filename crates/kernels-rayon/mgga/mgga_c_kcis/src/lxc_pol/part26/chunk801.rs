//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 801/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk801(t1331: f64, t2331: f64, t251: f64, t3977: f64, t11407: f64, t250: f64, t3106: f64, t461: f64, t453: f64, t1314: f64, t3897: f64, t455: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11455 = t2331 * t1331;
    let t11462 = t251 * t3977;
    let t11479 = 0.93932222222222222223e0_f64 * t11407;
    let t11481 = t250 * t3106 * t461;
    let t11482 = 0.36793333333333333333e0_f64 * t11481;
    let t11491 = 1.0_f64/pow_3_2(t453);
    let t11512 = 1.0_f64 / t3897 / t1314;
    let t11513 = t455 * t11512;
    (t11455, t11462, t11479, t11481, t11482, t11491, t11513)
}
