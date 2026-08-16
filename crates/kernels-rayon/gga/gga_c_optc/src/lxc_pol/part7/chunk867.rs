//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 867/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk867(t352: f64, t870: f64, t301: f64, t8287: f64, t2329: f64, t303: f64, t280: f64, t1765: f64, t362: f64, t287: f64, t8: f64, t2320: f64, t2344: f64, t2433: f64, t2438: f64, t277: f64, t4038: f64, t7325: f64, t7332: f64, t7335: f64, t7339: f64, t7346: f64, t7348: f64, t7499: f64, t7507: f64, t7509: f64, t7608: f64, t8267: f64, t8273: f64, t8277: f64, t8280: f64, t8283: f64, t95: f64, t962: f64, t984: f64, t989: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8288 = t352 * t870;
    let t8289 = t301 * t301;
    let t8291 = t8287 * t8288 * t8289;
    let t8292 = t2329 * t303;
    let t8294 = 1.0_f64 / t280 / t8292;
    let t8297 = t1765 * t362;
    let t8298 = t8294 * t8 * t287 * t8297;
    let t8303 = -50.0_f64 / 3.0_f64 * t7325 * t2438 + t7332 - 4.0_f64 * t2320 * t989 + t7335 / 2.0_f64 - t7339 + t7346 + t7348 + 0.25844881434903430496e-2_f64 * t95 * t277 * t8267 * t962 + 2.0_f64 / 3.0_f64 * t4038 * t8273 + 50.0_f64 / 27.0_f64 * t2433 * t8277 - t7509 - t7608 - t7499 - t7507 + 100.0_f64 / 27.0_f64 * t8280 * t2438 - t8283 / 3.0_f64 + 4000000.0_f64 / 243.0_f64 * t8291 * t8298 + 44.0_f64 / 3.0_f64 * t984 * t2344;
    (t8289, t8291, t8294, t8297, t8298, t8303)
}
