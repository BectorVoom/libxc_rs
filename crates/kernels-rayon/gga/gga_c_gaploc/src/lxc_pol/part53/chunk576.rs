//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 576/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk576(t10163: f64, t9074: f64, t123: f64, t7887: f64, t2326: f64, t3351: f64, t484: f64, t2854: f64, t6509: f64, t6320: f64, t2268: f64, t3327: f64, t6305: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10164 = t9074 * t10163;
    let t10165 = 0.11856252764865062333e-2_f64 * t10164;
    let t10166 = t7887 * t123;
    let t10167 = t10166 * t2326;
    let t10168 = t9074 * t10167;
    let t10169 = 0.35568758294595186999e-2_f64 * t10168;
    let t10175 = t484 * t3351;
    let t10176 = 0.15808337019820083111e-2_f64 * t10175;
    let t10177 = t2854 * t6509;
    let t10178 = t6320 * t10177;
    let t10180 = 0.17073003981405689759e0_f64 * t2268 * t10178;
    let t10184 = 0.28455006635676149599e-1_f64 * t6305 * t3327;
    (t10165, t10166, t10169, t10176, t10177, t10180, t10184)
}
