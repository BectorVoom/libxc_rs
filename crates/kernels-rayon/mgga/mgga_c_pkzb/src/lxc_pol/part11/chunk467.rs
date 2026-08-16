//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 467/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk467(t2172: f64, t858: f64, t862: f64, t361: f64, t861: f64) -> (f64, f64, f64, f64) {
    let t2246 = 0.22831111111111111111e-1_f64 * t2172;
    let t2252 = t858 * t862;
    let t2255 = t861 * t361;
    let t2256 = 1.0_f64 / t2255;
    (t2246, t2252, t2255, t2256)
}
