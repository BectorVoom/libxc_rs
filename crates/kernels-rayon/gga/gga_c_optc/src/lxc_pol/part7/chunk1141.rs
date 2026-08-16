//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1141/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1141(t8306: f64, t984: f64, t8381: f64, t993: f64, t2473: f64, t7341: f64, t7730: f64, t845: f64) -> (f64, f64, f64, f64) {
    let t23781 = t984 * t8306;
    let t23783 = t8381 * t993;
    let t23788 = 0.62336721237753107879e3_f64 * t845 * t7341 * t2473 * t7730;
    let t23789 = t2473 * t2473;
    (t23781, t23783, t23788, t23789)
}
