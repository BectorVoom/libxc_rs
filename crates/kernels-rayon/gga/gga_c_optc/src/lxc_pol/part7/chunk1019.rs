//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1019/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1019(t2022: f64, t6: f64, t127: f64, t2067: f64, t2024: f64, t2030: f64, t6889: f64, t6799: f64, t6885: f64, t2029: f64, t6875: f64, t6881: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22245 = t2022 * t2022;
    let t22246 = t6 * t22245;
    let t22247 = t22246 * t127;
    let t22251 = t2067 * t2067;
    let t22252 = t6 * t22251;
    let t22253 = t22252 * t2024;
    let t22257 = t22252 * t127;
    let t22261 = t2030 * t6889;
    let t22263 = t6799 * t6885;
    let t22265 = t6875 * t2029;
    let t22266 = t22265 * t6881;
    (t22245, t22246, t22247, t22251, t22253, t22257, t22261, t22263, t22266)
}
