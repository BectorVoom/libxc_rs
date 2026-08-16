//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 867/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk867(t1971: f64, t236: f64, t6113: f64, t7365: f64, t128: f64, t1867: f64, t118: f64, t1184: f64, t1986: f64, t35191: f64, t495: f64, t6182: f64, t7453: f64) -> (f64, f64, f64, f64) {
    let t44584 = t7365 * t1971 * t236 * t6113;
    let t44586 = t128 * t1867;
    let t44589 = t1986 * t118 * t44586 * t1184;
    let t44590 = t35191 * t44589;
    let t44595 = t7453 * t1971 * t236 * t6182 * t495;
    (t44584, t44586, t44590, t44595)
}
